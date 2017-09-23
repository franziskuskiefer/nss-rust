use std::env;
use std::process::Command;

fn main() {
    let nss_dir = env::var("NSS_DIR").unwrap();
    let lib_dir = env::var("NSS_LIB_DIR").unwrap();

    // Build NSS.
    // Command::new("build.sh")
    //     .current_dir(nss_dir.clone())
    //     .status()
    //     .unwrap();

    println!("cargo:rustc-link-search=native={}", lib_dir);
    println!("cargo:rustc-link-lib=dylib=nss3");
    println!("cargo:rustc-link-lib=dylib=ssl3");
}
