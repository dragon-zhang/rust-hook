use std::os::raw::c_uint;

pub fn sleep(secs: c_uint) -> c_uint {
    unsafe { libc::sleep(secs) }
}