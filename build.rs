fn main() {
    if cfg!(feature = "enable") {
        println!("cargo:warning=RealTime Sanitizer is enabled.");
    }
}
