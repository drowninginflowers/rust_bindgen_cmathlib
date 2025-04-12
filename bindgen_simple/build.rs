use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("c_mathlib")
        .canonicalize()
        .expect("cannot canonicalize path");

    let header_path = libdir_path.join("mathlib.h");

    println!("cargo::rerun-if-changed={}", header_path.display());
    println!(
        "cargo::rerun-if-changed={}",
        libdir_path.join("mathlib.c").display()
    );

    cc::Build::new()
        .file(libdir_path.join("mathlib.c"))
        .include(&libdir_path)
        .compile("mathlib");

    println!("cargo::rustc-link-lib=mathlib");
    println!(
        "cargo::rustc-link-search=native={}",
        env::var("OUT_DIR").unwrap()
    );

    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
