use super::tools::LIB;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Arc;
pub fn Sm4Enc(key: &str, msg: &str) -> anyhow::Result<String> {
    let k_c_str = CString::new(key)?;
    let msg_c_str = CString::new(msg)?;

    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let sm4enc: libloading::Symbol<
            unsafe extern "C" fn(k: *const c_char, msg: *const c_char) -> *mut c_char,
        > = lib.get(b"Sm4Enc")?;
        let raw = sm4enc(k_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn Sm4Dec(key: &str, msg: &str) -> anyhow::Result<String> {
    let k_c_str = CString::new(key)?;
    let msg_c_str = CString::new(msg)?;

    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let sm4dec: libloading::Symbol<
            unsafe extern "C" fn(k: *const c_char, msg: *const c_char) -> *mut c_char,
        > = lib.get(b"Sm4Dec")?;
        let raw = sm4dec(k_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
