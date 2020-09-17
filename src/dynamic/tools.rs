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
    let dir1 = std::env::var("OUT_DIR");
    if let Err(e) = dir1 {
        panic!("can not find out dir")
    }
    let dir = dir1.unwrap();
    dbg!(&dir);
    #[cfg(target_os = "windows")]
    {
        let f = format!("{}/libgosm.dll", &dir);
        // String::from("libgosm.dll")
        f
    }

    #[cfg(target_os = "linux")]
    {
        let f = format!("{}/libgosm.so", &dir);
        f
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
pub fn test() -> anyhow::Result<()> {
    // Hi();
    // let b = dynamic::sm2::GenKeyPair()?;
    // dbg!(b);
    // let msg = "adsfads";
    // let a = dynamic::sm3::Sm3(msg)?;
    // dbg!(a);
    // let c = dynamic::sm2::Sm2Enc("pk.pem", "asdfa")?;
    // let e = dynamic::sm2::Sm2Dec("sk.pem", &c)?;
    // dbg!(e);
    // let f = dynamic::sm2::Sm2Sign("sk.pem", "asdfasdf")?;
    // let g = dynamic::sm2::Sm2Verify("pk.pem", "asdfasdf", &f)?;
    // dbg!(g);
    // let k = "1234567890abcdef";
    // let h = dynamic::sm4::Sm4Enc(k, "asdfa")?;
    // dbg!(&h);
    // let i = dynamic::sm4::Sm4Dec(k, &h)?;
    // dbg!(&i);
    Ok(())
}
