fn main() {
    let enable = cfg!(feature = "enable");
    let supported_os = cfg!(any(target_os = "macos", target_os = "linux"));

    if enable && !supported_os {
        println!("cargo:warning=RTSan is only supported on macOS and Linux.");
    }
}
