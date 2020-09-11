use super::tools::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn Sm3(msg: &str) -> anyhow::Result<String> {
    // let go_str_msg = str_2_go_str(msg)?;
    let c_str = CString::new(msg)?;

    let file = load_dynamic();
    let lib = libloading::Library::new(file)?;
    unsafe {
        let arg = c_str.as_ptr();
        let sm3: libloading::Symbol<unsafe extern "C" fn(msg: *const c_char) -> *mut c_char> =
            lib.get(b"Sm3")?;
        let raw = sm3(arg);
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        Ok(res)
    }
}
