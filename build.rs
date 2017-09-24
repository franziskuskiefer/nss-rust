extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut bindings = bindgen::builder();

    let include_dirs = env::var("NSS_INCLUDE_DIR").expect("Please set NSS_INCLUDE_DIR");
    let include_dirs = include_dirs.split(' ');
    for dir in include_dirs {
        let tmp = "-I".to_string() + &dir;
        bindings = bindings.clang_arg(tmp);
    }
    let nss_lib_dir = env::var("NSS_LIB_DIR").expect("Please set NSS_LIB_DIR");

    println!("cargo:rustc-link-search=native={}", nss_lib_dir);
    println!("cargo:rustc-link-lib=dylib=nspr4");
    println!("cargo:rustc-link-lib=dylib=nss3");
    println!("cargo:rustc-link-lib=dylib=ssl3");
    println!("cargo:rustc-link-lib=dylib=nssutil3");
    println!("cargo:rustc-link-lib=dylib=nssckbi");
    println!("cargo:rustc-link-lib=dylib=freebl3");
    println!("cargo:rustc-link-lib=dylib=nssdbm3");
    println!("cargo:rustc-link-lib=dylib=softokn3");

    let bindings = bindings.header("wrapper.h").generate().expect(
        "Unable to generate bindings",
    );

    // $OUT_DIR is set by cargo.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
