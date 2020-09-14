use once_cell::sync::{Lazy, OnceCell};
use std::sync::{Arc, Mutex};
pub static LIB: Lazy<Arc<Mutex<libloading::Library>>> = Lazy::new(|| {
    let file = load_dynamic();
    let lib = libloading::Library::new(file).unwrap();
    let res1 = Mutex::new(lib);
    let res2 = Arc::new(res1);
    res2
});

pub fn load_dynamic() -> String {
    #[cfg(target_os = "windows")]
    {
        String::from("libgosm.dll")
    }

    #[cfg(target_os = "linux")]
    {
        String::from("libgosm.so")
    }
}

pub fn Hi() -> anyhow::Result<()> {
    // let file = load_dynamic();
    // let lib = libloading::Library::new(file)?;
    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn()> = lib.get(b"Hi")?;
        func();
        Ok(())
    }
}
