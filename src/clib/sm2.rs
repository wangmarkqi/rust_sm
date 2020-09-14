use super::tools::LIB;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Arc;
pub fn GenKeyPair() -> anyhow::Result<String> {
    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let gen: libloading::Symbol<unsafe extern "C" fn() -> *mut c_char> =
            lib.get(b"GenKeyPair")?;
        let raw = gen();

        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn Sm2Enc(pkpath: &str, message: &str) -> anyhow::Result<String> {
    let pk_c_str = CString::new(pkpath)?;
    let msg_c_str = CString::new(message)?;

    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let sm2enc: libloading::Symbol<
            unsafe extern "C" fn(pk: *const c_char, msg: *const c_char) -> *mut c_char,
        > = lib.get(b"Sm2Enc")?;

        let raw = sm2enc(pk_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn Sm2Dec(skpath: &str, message: &str) -> anyhow::Result<String> {
    let sk_c_str = CString::new(skpath)?;
    let msg_c_str = CString::new(message)?;

    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let sm2dec: libloading::Symbol<
            unsafe extern "C" fn(sk: *const c_char, msg: *const c_char) -> *mut c_char,
        > = lib.get(b"Sm2Dec")?;

        let raw = sm2dec(sk_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn Sm2Sign(skpath: &str, message: &str) -> anyhow::Result<String> {
    let sk_c_str = CString::new(skpath)?;
    let msg_c_str = CString::new(message)?;

    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let sm2sign: libloading::Symbol<
            unsafe extern "C" fn(sk: *const c_char, msg: *const c_char) -> *mut c_char,
        > = lib.get(b"Sm2Sign")?;

        let raw = sm2sign(sk_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn Sm2Verify(pkpath: &str, message: &str, sign: &str) -> anyhow::Result<String> {
    let sk_c_str = CString::new(pkpath)?;
    let msg_c_str = CString::new(message)?;
    let sign_c_str = CString::new(sign)?;
    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let sm2ver: libloading::Symbol<
            unsafe extern "C" fn(
                sk: *const c_char,
                msg: *const c_char,
                sign: *const c_char,
            ) -> *mut c_char,
        > = lib.get(b"Sm2Verify")?;

        let raw = sm2ver(sk_c_str.as_ptr(), msg_c_str.as_ptr(), sign_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
