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

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .file(shim_src.join("bridge.cpp"))
        .file(shim_src.join("force_link.cpp"))
        .include(&skia_dir)
        .include(&shim_include)
        .warnings(false)
        .compile("skialin_bridge");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++20")
        .clang_arg("-D_ALLOW_COMPILER_AND_STL_VERSION_MISMATCH")
        .clang_arg(format!("-I{}", skia_dir.display()))
        .clang_arg(format!("-I{}", shim_include.display()))
        .allowlist_type("Sk.*")
        .allowlist_function("Sk.*")
        .allowlist_function("skialin_bridge_.*")
        .allowlist_var("Sk.*")
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

fn link_skia(skia_dir: &Path) {
    let lib_dir = env::var("SKIALIN_SKIA_LIB_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| skia_dir.join("out/Release"));

    if !lib_dir.is_dir() {
        println!(
            "cargo:warning=skia-sys: no built Skia libs found at {} (set SKIALIN_SKIA_LIB_DIR once Skia is built); bindgen output is available but linking will fail",
            lib_dir.display()
        );
        return;
    }

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=skparagraph");
    println!("cargo:rustc-link-lib=static=skshaper");
    println!("cargo:rustc-link-lib=static=skunicode_core");
    println!("cargo:rustc-link-lib=static=skunicode_icu");
    println!("cargo:rustc-link-lib=static=skia");
    println!("cargo:rustc-link-lib=static=harfbuzz");
    println!("cargo:rustc-link-lib=static=icu");
    println!("cargo:rustc-link-lib=static=skcms");
    println!("cargo:rustc-link-lib=static=libpng");
    println!("cargo:rustc-link-lib=static=zlib");
    println!("cargo:rustc-link-lib=static=expat");

    if cfg!(target_os = "windows") {
        for lib in [
            "gdi32", "user32", "ole32", "advapi32", "usp10", "dwrite", "fontsub", "shlwapi", "rpcrt4", "opengl32",
        ] {
            println!("cargo:rustc-link-lib=dylib={lib}");
        }
    }
}
