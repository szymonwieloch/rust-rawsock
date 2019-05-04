use std::ffi::CStr;
use libc::{c_char, strerror};
use errno::errno;

pub fn cstr_to_string(txt: * const c_char) -> String {
    unsafe { CStr::from_ptr(txt) }.to_string_lossy().into_owned()
}

pub fn string_from_errno() -> String {
    cstr_to_string(
    unsafe{strerror(errno().into())}
    )
}