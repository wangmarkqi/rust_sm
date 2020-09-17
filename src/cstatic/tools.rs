use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "sm", kind = "static")]
extern "C" {
    fn Hi();
    fn GenKeyPair() -> *mut c_char;
    fn Sm2Enc(pk: *const c_char, msg: *const c_char) -> *mut c_char;
    fn Sm2Dec(sk: *const c_char, msg: *const c_char) -> *mut c_char;
    fn Sm2Sign(pk: *const c_char, msg: *const c_char) -> *mut c_char;
    fn Sm2Verify(sk: *const c_char, msg: *const c_char, sign: *const c_char) -> *mut c_char;
    fn Sm3(msg: *const c_char) -> *mut c_char;

    fn Sm4Enc(k: *const c_char, msg: *const c_char) -> *mut c_char;
    fn Sm4Dec(k: *const c_char, msg: *const c_char) -> *mut c_char;
}

pub fn hi() -> anyhow::Result<()> {
    unsafe {
        Hi();
    }
    Ok(())
}

pub fn genKeyPair() -> anyhow::Result<String> {
    unsafe {
        let raw = GenKeyPair();
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn sm2Enc(pkpath: &str, message: &str) -> anyhow::Result<String> {
    let pk_c_str = CString::new(pkpath)?;
    let msg_c_str = CString::new(message)?;

    unsafe {
        let raw = Sm2Enc(pk_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn sm2Dec(skpath: &str, message: &str) -> anyhow::Result<String> {
    let sk_c_str = CString::new(skpath)?;
    let msg_c_str = CString::new(message)?;

    unsafe {
        let raw = Sm2Dec(sk_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn sm2Sign(skpath: &str, message: &str) -> anyhow::Result<String> {
    let sk_c_str = CString::new(skpath)?;
    let msg_c_str = CString::new(message)?;

    unsafe {
        let raw = Sm2Sign(sk_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn sm2Verify(pkpath: &str, message: &str, sign: &str) -> anyhow::Result<String> {
    let pk_c_str = CString::new(pkpath)?;
    let msg_c_str = CString::new(message)?;
    let sign_c_str = CString::new(sign)?;
    unsafe {
        let raw = Sm2Verify(pk_c_str.as_ptr(), msg_c_str.as_ptr(), sign_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}

pub fn sm3(msg: &str) -> anyhow::Result<String> {
    let c_str = CString::new(msg)?;
    unsafe {
        let arg = c_str.as_ptr();
        let raw = Sm3(arg);
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn sm4Enc(key: &str, msg: &str) -> anyhow::Result<String> {
    let k_c_str = CString::new(key)?;
    let msg_c_str = CString::new(msg)?;

    unsafe {
        let raw = Sm4Enc(k_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
pub fn sm4Dec(key: &str, msg: &str) -> anyhow::Result<String> {
    let k_c_str = CString::new(key)?;
    let msg_c_str = CString::new(msg)?;

    unsafe {
        let raw = Sm4Dec(k_c_str.as_ptr(), msg_c_str.as_ptr());
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
