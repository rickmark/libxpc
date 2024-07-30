extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let plist = pkg_config::probe_library("libplist-2.0").unwrap();

    let bindings = bindgen::Builder::default()
        .clang_args(plist.include_paths.iter().map(|path| format!("-I{}", path.to_string_lossy())))
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}