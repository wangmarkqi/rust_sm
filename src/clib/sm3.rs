use super::tools::LIB;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::Arc;
pub fn Sm3(msg: &str) -> anyhow::Result<String> {
    // let go_str_msg = str_2_go_str(msg)?;
    let c_str = CString::new(msg)?;

    let mylib = Arc::clone(&LIB);
    let lib = mylib.lock().unwrap();
    unsafe {
        let arg = c_str.as_ptr();
        let sm3: libloading::Symbol<unsafe extern "C" fn(msg: *const c_char) -> *mut c_char> =
            lib.get(b"Sm3")?;
        let raw = sm3(arg);
        let res = CStr::from_ptr(raw).to_string_lossy().into_owned();
        // 这个不行 let res = CString::from_raw(raw).into_string()?;
        Ok(res)
    }
}
