use std::os::raw::c_void;

//被hook的系统函数
#[no_mangle]
pub extern "C" fn abs(i: i32) -> i32 {
    unsafe {
        println!("hooked {}", i);
        //fixme 如何正确地调用系统函数?
        let original = libc::dlsym(libc::RTLD_NEXT, "abs".as_ptr() as *const _)
            as *mut fn(i32) -> i32;
        (*original)(i)
        // i
    }
}