use std::env::var;

fn main() {
    #[cfg(feature = "user")]
    {
        if var("TARGET")
            .map(|t| t == "x86_64-pc-windows-gnu" || t == "i686-pc-windows-gnu")
            .unwrap_or(false)
        {
            println!("cargo:rustc-link-lib=ntdll");
        }
    }
}
