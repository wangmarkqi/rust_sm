use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the output path
    // let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = env::var("OUT_DIR").unwrap();
    println!("cargo:warning=MESSAGE");
    println!("cargo:warning=dir={:?}", &dir);
    println!("cargo:rustc-env={}={}", "OUT_DIR", &dir);
    // Copy *.dll & .lib to the output path
    let so: String = String::from("./gosm/libgosm.so");
    let dll: String = String::from("./gosm/libgosm.dll");
    let dll_dest_path = Path::new(&dir).join("libgosm.dll");
    let so_dest_path = Path::new(&dir).join("libgosm.so");
    let _so_result = fs::copy(so, so_dest_path);
    let _dll_result = fs::copy(dll, dll_dest_path);

    // Link Dynamsoft Barcode Reader.
    // println!("cargo:rustc-link-search={}", env!("DBR_LIB"));
    println!("cargo:rustc-link-search=all={}", &dir);
    println!("cargo:rustc-link-lib=dylib=gosm");
}
