use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("../c_mathlib")
        .canonicalize()
        .expect("cannot canonicalize path");

    let headers_path = libdir_path.join("mathlib.h");
    let headers_path_str = headers_path.to_str().expect("Path is not a valid string");

    let obj_path = libdir_path.join("mathlib.o");
    let lib_path = libdir_path.join("libmathlib.a");

    println!("cargo::rustc-link-search={}", libdir_path.to_str().unwrap());

    println!("cargo::rustc-link-lib=mathlib");

    if !std::process::Command::new("clang")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(libdir_path.join("mathlib.c"))
        .output()
        .expect("could not spawn `clang`")
        .status
        .success()
    {
        panic!("could not compile object file");
    }

    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("could not spawn `ar`")
        .status
        .success()
    {
        panic!("could not emit library file");
    }

    let bindings = bindgen::Builder::default()
        .header(headers_path_str)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}
