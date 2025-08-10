fn main() {
    println!(r"cargo:rustc-link-search=resources");
    println!(r"cargo:rustc-link-lib=static=SDL3");
    println!(r"cargo:rustc-link-arg=--exclude-libs=libSDL3.a");
}
