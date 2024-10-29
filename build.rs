extern crate cbindgen;

use std::env;

fn cbindgen_generate() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo::rustc-cdylib-link-arg=-fuse-ld=lld");
    // println!(
    //     "cargo::rustc-cdylib-link-arg=-Wl,--version-script={}/libdatrie.map",
    //     crate_dir
    // );

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include_guard("THAI_THAILIB_H")
        .with_include_version(true)
        .with_cpp_compat(true)
        .with_include("stddef.h")
        .rename_item("BrkOp", "brk_op_t")
        .rename_item("BrkClass", "brk_class_t")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("include/thailib.h");
}

fn main() {
    if env::var("CARGO_FEATURE_CFFI").is_ok() {
        cbindgen_generate();
    }
}
