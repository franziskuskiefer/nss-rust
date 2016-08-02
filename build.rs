use std::process::Command;
use std::env;

fn main() {
    let nss_dir = env::var("NSS_DIR").unwrap();
    let lib_dir = env::var("NSS_LIB_DIR").unwrap();

    // .arg("nss_clean_all")
    // Command::new("make").env("NSDISTMODE", "copy").arg("nss_build_all").current_dir(nss_dir.clone()).status().unwrap();

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=nss3");
    println!("cargo:rustc-link-lib=dylib=ssl3");
}