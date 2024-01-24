use pkg_config::Library;

const LIBRARY_NAME: &str = "editorconfig";

// Technically libeditorconfig v0.12.2 already supports pkg-config:
// https://github.com/editorconfig/editorconfig-core-c/releases/tag/v0.12.2
const MIN_VERSION: &str = "0.12.5";
const MAX_VERSION: &str = "1.0.0";

fn main() {
    let err_msg = format!("Unable to find library {} >= {}", LIBRARY_NAME, MIN_VERSION);
    let lib = pkg_config::Config::new()
        .range_version(MIN_VERSION..MAX_VERSION)
        .probe(LIBRARY_NAME)
        .expect(&err_msg);
    gen_bindings(lib);
}

fn gen_bindings(lib: Library) {
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

    // Write bindings to `$OUT_DIR/bindings.rs`
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
