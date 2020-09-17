use std::env;
use std::fs;
use std::path::Path;

fn main() {
    copy_or_link();
}

#[cfg(target_os = "windows")]
fn copy_or_link() {
    let out = env::var("OUT_DIR").unwrap();
    println!("cargo:warning=dir={:?}", &out);
    println!("cargo:rustc-env={}={}", "OUT_DIR", &out);

    let src: String = String::from("./gosm/libgosm.dll");
    let dest = Path::new(&out).join("libgosm.dll");
    fs::copy(src, dest);
}

#[cfg(target_os = "linux")]
fn copy_or_link() {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap();
    let go_dir = Path::new(&root).join("gosm");
    // 必须用这个库操作路径，不然找不到
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
