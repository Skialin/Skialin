use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-arg=/OPT:REF");
            println!("cargo:rustc-link-arg=/OPT:ICF");
        }
        "linux" | "android" => {
            let map = manifest_dir.join("exports/skialin_jni.map");
            println!("cargo:rustc-link-arg=-Wl,--gc-sections");
            println!("cargo:rustc-link-arg=-Wl,--version-script={}", map.display());
            println!("cargo:rerun-if-changed={}", map.display());
        }
        "macos" => {
            let list = manifest_dir.join("exports/skialin_jni.exports");
            println!("cargo:rustc-link-arg=-Wl,-ld_classic");
            println!("cargo:rustc-link-arg=-Wl,-dead_strip");
            println!("cargo:rustc-link-arg=-Wl,-exported_symbols_list,{}", list.display());
            println!("cargo:rerun-if-changed={}", list.display());
        }
        _ => {}
    }
}
