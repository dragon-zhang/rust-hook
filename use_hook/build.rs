fn main() {
    //hook就是这样玩的
    println!("cargo:rustc-link-lib=dylib=hook");
}