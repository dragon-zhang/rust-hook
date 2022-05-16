fn main() {
    unsafe {
        //must add a build.rs
        //sleep 1s
        println!("{}", libc::sleep(1));
        //will be hooked
        println!("{}", use_hook::sleep(1));
    }
}
