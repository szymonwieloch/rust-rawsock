use std::ffi::CStr;
use libc::{c_char, strerror, c_int};
use errno::errno;

pub fn cstr_to_string(txt: * const c_char) -> String {
    unsafe { CStr::from_ptr(txt) }.to_string_lossy().into_owned()
}

pub fn string_from_errno() -> String {
    cstr_to_string(
    unsafe{strerror(errno().into())}
    )
}

pub fn string_from_err_code(code: c_int) -> String {
    let corrected = if code <0 {
        -code
    } else {
        code
    };
    cstr_to_string(
        unsafe{strerror(corrected)}
    )
}