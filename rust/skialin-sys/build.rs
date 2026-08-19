use std::env;
use std::path::{Path, PathBuf};

fn skia_dir() -> PathBuf {
    if let Ok(dir) = env::var("SKIALIN_SKIA_DIR") {
        return PathBuf::from(dir);
    }
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("../../external/skia")
}

fn shim_include_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("../../native-shim/include")
}

fn shim_src_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("../../native-shim/src")
}

fn main() {
    let skia_dir = skia_dir();
    let shim_include = shim_include_dir();
    let shim_src = shim_src_dir();

    if !skia_dir.join("include/core/SkCanvas.h").is_file() {
        panic!(
            "Skia checkout not found at {}. Set SKIALIN_SKIA_DIR or clone google/skia into external/skia.",
            skia_dir.display()
        );
    }

    println!("cargo:rerun-if-env-changed=SKIALIN_SKIA_DIR");
    println!("cargo:rerun-if-env-changed=SKIALIN_SKIA_LIB_DIR");
    println!("cargo:rerun-if-changed={}", shim_include.display());
    println!("cargo:rerun-if-changed={}", shim_src.display());
    println!("cargo:rerun-if-changed=wrapper.h");

    // The "pathops" GN target is a source_set, not a static_library, so it
    // never produces its own .lib and isn't pulled into skia.lib either
    // (nothing in this minimal build's dependency graph links it). Rather
    // than patch the Skia GN build, compile Skia's own pathops sources
    // directly as part of this crate's C++ build.
    let pathops_dir = skia_dir.join("src/pathops");
    let pathops_sources = [
        "SkAddIntersections.cpp",
        "SkDConicLineIntersection.cpp",
        "SkDCubicLineIntersection.cpp",
        "SkDCubicToQuads.cpp",
        "SkDLineIntersection.cpp",
        "SkDQuadLineIntersection.cpp",
        "SkIntersections.cpp",
        "SkOpAngle.cpp",
        "SkOpBuilder.cpp",
        "SkOpCoincidence.cpp",
        "SkOpContour.cpp",
        "SkOpCubicHull.cpp",
        "SkOpEdgeBuilder.cpp",
        "SkOpSegment.cpp",
        "SkOpSpan.cpp",
        "SkPathOpsAsWinding.cpp",
        "SkPathOpsCommon.cpp",
        "SkPathOpsConic.cpp",
        "SkPathOpsCubic.cpp",
        "SkPathOpsCurve.cpp",
        "SkPathOpsDebug.cpp",
        "SkPathOpsLine.cpp",
        "SkPathOpsOp.cpp",
        "SkPathOpsQuad.cpp",
        "SkPathOpsRect.cpp",
        "SkPathOpsSimplify.cpp",
        "SkPathOpsTSect.cpp",
        "SkPathOpsTightBounds.cpp",
        "SkPathOpsTypes.cpp",
        "SkPathOpsWinding.cpp",
        "SkPathWriter.cpp",
        "SkReduceOrder.cpp",
    ];

    let defines = skia_defines(&skia_lib_dir(&skia_dir));

    let shim_build = || {
        let mut build = cc::Build::new();
        build
            .cpp(true)
            .std("c++20")
            .include(&skia_dir)
            .include(&shim_include)
            .include(skia_dir.join("include/third_party/vulkan"))
            .define("SK_USE_INTERNAL_VULKAN_HEADERS", None)
            .warnings(false);
        for define in &defines {
            let mut parts = define.splitn(2, '=');
            build.define(parts.next().unwrap(), parts.next());
        }
        // Skia applies gn/skia/BUILD.gn's "no_rtti" config to every one of its
        // own targets, so libskia has no typeinfo for any Skia class. Deriving
        // from SkDrawable with RTTI on leaves the subclass typeinfo referencing
        // a base typeinfo that was never emitted; the Itanium ABI linkers say
        // so ("undefined symbol: typeinfo for SkDrawable"), MSVC papers over it
        // by emitting the whole hierarchy descriptor per TU.
        if build.get_compiler().is_like_msvc() {
            build.flag("/GR-");
        } else {
            build.flag("-fno-rtti");
        }
        build
    };

    shim_build()
        .file(shim_src.join("bridge.cpp"))
        .files(pathops_sources.iter().map(|f| pathops_dir.join(f)))
        .compile("skialin_bridge");

    // force_link.cpp exists to give Skia's header-defined (inline) member
    // functions out-of-line copies, because bindgen's generate_inline_functions
    // binds them as ordinary extern calls. Optimizing this translation unit
    // defeats the point: clang and gcc inline each call and then drop the now
    // unreferenced linkonce_odr definition, leaving the JNI library with
    // undefined symbols (a hard link error on macOS, a dlopen failure on
    // Linux). MSVC keeps them, which is why only Windows ever linked.
    shim_build()
        .opt_level(0)
        .file(shim_src.join("force_link.cpp"))
        .compile("skialin_force_link");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
        .clang_arg("-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH")
        .clang_arg(format!("-I{}", skia_dir.display()))
        .clang_arg(format!("-I{}", shim_include.display()))
        .clang_arg(format!("-I{}", skia_dir.join("include/third_party/vulkan").display()))
        .clang_arg("-DSK_USE_INTERNAL_VULKAN_HEADERS")
        .clang_args(defines.iter().map(|define| format!("-D{define}")))
        .allowlist_type("Sk.*")
        .allowlist_function("Sk.*")
        .allowlist_function("skialin_bridge_.*")
        .allowlist_var("Sk.*")
        .allowlist_var("VK_.*")
        .opaque_type("std::.*")
        .layout_tests(false)
        .derive_default(true)
        .enable_cxx_namespaces()
        .generate_inline_functions(true)
        .generate()
        .expect("failed to generate skialin-sys bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("failed to write bindings.rs");

    link_skia(&skia_dir);
}

fn skia_lib_dir(skia_dir: &Path) -> PathBuf {
    env::var("SKIALIN_SKIA_LIB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| skia_dir.join("out/Release"))
}

/// The feature macros Skia compiled its own translation units with, read back
/// out of the ninja files `gn gen` wrote. Guessing at this list by hand is how
/// you end up with a shim whose idea of a Skia class disagrees with libskia's:
/// `NDEBUG` alone decides `SK_DEBUG` vs `SK_RELEASE`, which adds `SkDEBUGCODE`
/// members to public classes and asserts to their inline methods, and the
/// backend switches (`SK_GANESH`, `SK_GRAPHITE`, `SK_GL`, `SK_VULKAN`) gate
/// declarations inside public headers. `*_IMPLEMENTATION` is the one family to
/// drop -- it flips `SK_API` from import to export, and only Skia's own build
/// should set it.
fn skia_defines(lib_dir: &Path) -> Vec<String> {
    let ninja_files = [
        "obj/skia.ninja",
        "obj/modules/skparagraph/skparagraph.ninja",
        "obj/modules/skshaper/skshaper.ninja",
        "obj/modules/skunicode/skunicode_core.ninja",
        "obj/modules/svg/svg.ninja",
        "obj/modules/skottie/skottie.ninja",
    ];

    let mut defines: Vec<String> = Vec::new();
    for ninja_file in ninja_files {
        let path = lib_dir.join(ninja_file);
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        println!("cargo:rerun-if-changed={}", path.display());
        let Some(line) = text.lines().find(|line| line.trim_start().starts_with("defines = ")) else { continue };
        for define in line.split_whitespace().filter_map(|token| token.strip_prefix("-D")) {
            let name = define.split('=').next().unwrap_or(define);
            let ours = name == "NDEBUG" || name == "GPU_TEST_UTILS" || name.starts_with("SK");
            if !ours || name.ends_with("_IMPLEMENTATION") {
                continue;
            }
            if !defines.iter().any(|existing| existing == define) {
                defines.push(define.to_string());
            }
        }
    }

    if !defines.iter().any(|define| define == "NDEBUG") {
        println!(
            "cargo:warning=skialin-sys: could not read Skia's own -D flags from {}; falling back to NDEBUG only, which risks an ABI mismatch with libskia",
            lib_dir.display()
        );
        defines.push("NDEBUG".to_string());
    }
    defines
}

fn link_skia(skia_dir: &Path) {
    let lib_dir = skia_lib_dir(skia_dir);

    if !lib_dir.is_dir() {
        println!(
            "cargo:warning=skia-sys: no built Skia libs found at {} (set SKIALIN_SKIA_LIB_DIR once Skia is built); bindgen output is available but linking will fail",
            lib_dir.display()
        );
        return;
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=skparagraph");
    println!("cargo:rustc-link-lib=static=svg");
    println!("cargo:rustc-link-lib=static=skottie");
    println!("cargo:rustc-link-lib=static=skresources");
    println!("cargo:rustc-link-lib=static=sksg");
    println!("cargo:rustc-link-lib=static=jsonreader");
    println!("cargo:rustc-link-lib=static=skshaper");
    println!("cargo:rustc-link-lib=static=skunicode_core");
    println!("cargo:rustc-link-lib=static=skunicode_icu");
    println!("cargo:rustc-link-lib=static=skia");
    println!("cargo:rustc-link-lib=static=harfbuzz");
    println!("cargo:rustc-link-lib=static=icu");
    println!("cargo:rustc-link-lib=static=skcms");

    let gn_lib_target = |name: &str| -> String {
        if cfg!(target_os = "windows") {
            name.to_string()
        } else {
            name.strip_prefix("lib").unwrap_or(name).to_string()
        }
    };
    println!("cargo:rustc-link-lib=static={}", gn_lib_target("libpng"));
    println!("cargo:rustc-link-lib=static=zlib");
    println!("cargo:rustc-link-lib=static=expat");
    println!("cargo:rustc-link-lib=static={}", gn_lib_target("libjpeg"));
    println!("cargo:rustc-link-lib=static={}", gn_lib_target("libwebp"));
    println!("cargo:rustc-link-lib=static={}", gn_lib_target("libwebp_sse41"));
    println!("cargo:rustc-link-lib=static=wuffs");

    if cfg!(target_os = "windows") {
        for lib in [
            "gdi32", "user32", "ole32", "advapi32", "usp10", "dwrite", "fontsub", "shlwapi", "rpcrt4", "opengl32",
        ] {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
    } else if cfg!(target_os = "macos") {
        for framework in ["AppKit", "ApplicationServices", "CoreFoundation", "CoreGraphics", "CoreText"] {
            println!("cargo:rustc-link-lib=framework={framework}");
        }
    } else if cfg!(target_os = "linux") {
        for lib in ["fontconfig", "freetype"] {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
    }

    copy_icu_data(&lib_dir);
}

/// SkLoadICU() (third_party/icu/SkLoadICU.cpp) looks for icudtl.dat next to
/// the module containing Skia's own compiled code, i.e. next to whatever
/// binary or dylib this static lib ends up linked into. For `cargo test`
/// that's target/{profile}/deps; copy proactively so tests don't need a
/// manual step. (The JNI .dll needs its own copy alongside it too --
/// build.gradle.kts's copyNativeLib task handles that.)
fn copy_icu_data(lib_dir: &Path) {
    let src = lib_dir.join("icudtl.dat");
    if !src.is_file() {
        return;
    }
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // OUT_DIR is target/{profile}/build/skialin-sys-{hash}/out
    let Some(profile_dir) = out_dir.ancestors().nth(3) else { return };
    for dest_dir in [profile_dir.to_path_buf(), profile_dir.join("deps")] {
        if dest_dir.is_dir() {
            let _ = std::fs::copy(&src, dest_dir.join("icudtl.dat"));
        }
    }
}
