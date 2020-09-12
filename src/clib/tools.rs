pub fn load_dynamic() -> String {
    #[cfg(target_os = "windows")]
    {
        // String::from("./gosm/libgosm.dll")
        String::from("libgosm.dll")
    }

    #[cfg(target_os = "linux")]
    {
        // String::from("./gosm/libgosm.so")
        String::from("libgosm.so")
    }
}

pub fn Hi() -> anyhow::Result<()> {
    let file = load_dynamic();
    dbg!(&file);
    let lib = libloading::Library::new(file)?;
    unsafe {
        let func: libloading::Symbol<unsafe extern "C" fn()> = lib.get(b"Hi")?;
        func();
        Ok(())
    }
}
