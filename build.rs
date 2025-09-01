fn main() {
    println!("cargo:rustc-env=CARGO_CFG_TARGET_FEATURE=+crt-static");

    #[cfg(debug_assertions)]
    {
        println!("cargo:rustc-link-arg=/DEBUG:FASTLINK");
        println!("cargo:rustc-link-arg=/OPT:NOREF");
    }
}
