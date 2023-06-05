fn main() {
    #[cfg(feature = "user")]
    {
        println!("cargo:rustc-link-lib=ntdll");
    }
}
