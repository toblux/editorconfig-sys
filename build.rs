// This build script currently assumes that `libeditorconfig >= 0.12.5` is
// installed and that `pkg-config` can find it.

use pkg_config::{Config, Library};

const LIBRARY_NAME: &str = "editorconfig";

// Technically libeditorconfig v0.12.2 already supports pkg-config:
// https://github.com/editorconfig/editorconfig-core-c/releases/tag/v0.12.2
const MIN_VERSION: &str = "0.12.5";

fn main() {
    if let Ok(lib) = Config::new()
        .atleast_version(MIN_VERSION)
        .probe(LIBRARY_NAME)
    {
        if cfg!(feature = "buildtime-bindgen") {
            gen_bindings(lib);
        }
    } else {
        eprintln!("Unable to find lib {} >= {}", LIBRARY_NAME, MIN_VERSION);
    }
}

fn gen_bindings(lib: Library) {
    use std::{env, path::PathBuf};

    let include_paths = lib
        .include_paths
        .iter()
        .map(|path| format!("-I{}", path.to_string_lossy()));

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(include_paths)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings");

    // Write auto-generated bindings to `$OUT_DIR/bindings.rs`
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
