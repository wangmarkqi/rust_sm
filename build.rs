use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let dir = env::var("OUT_DIR").unwrap();
    println!("cargo:warning=dir={:?}", &dir);
    println!("cargo:rustc-env={}={}", "OUT_DIR", &dir);

    copy_dynamic(&dir);
}

#[cfg(target_os = "windows")]
fn copy_dynamic(out: &str) {
    let src: String = String::from("./gosm/libgosm.dll");
    let dest = Path::new(out).join("libgosm.dll");
    fs::copy(src, dest);
}

#[cfg(target_os = "linux")]
fn copy_dynamic(out: &str) {
    let src: String = String::from("./gosm/libgosm.so");
    let dest = Path::new(out).join("libgosm.so");
    fs::copy(src, dest);
}

#[cfg(target_os = "linux")]
fn link_static() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let go_dir = Path::new(&root).join("gosm");
    let lib_dir = dunce::canonicalize(go_dir)
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();
    println!("cargo:rustc-link-search=native={}", &lib_dir);
    println!("cargo:warning=MESSAGE**********{}", &lib_dir);
    let lib_name = "sm";
    println!("cargo:rustc-link-lib=static={}", lib_name);
}
