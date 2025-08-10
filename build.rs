fn main() {
    println!("cargo:rustc-link-search=resources");
    println!("cargo:rustc-link-lib=static=SDL3");
    println!("cargo:rustc-link-arg=--exclude-libs=libSDL3.a");
    println!("cargo:rustc-link-arg=--shared");
    println!("cargo:rustc-link-arg=--retain-symbols-file=resources/retain.txt");
    println!("cargo:rustc-link-arg=--gc-sections");
    println!("cargo:rustc-link-arg=--strip-debug");
    println!("cargo:rustc-link-arg=--exclude-libs=libSDL3.a");
    println!("cargo:rustc-link-arg=--discard-locals");
    println!("cargo:rustc-link-arg=--entry=main");
}
