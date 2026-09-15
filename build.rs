fn main() {
    println!("cargo::rustc-check-cfg=cfg(rtsan_enabled)");
    println!("cargo::rustc-check-cfg=cfg(rtsan_supported)");

    const RTSAN_ENV_VAR: &str = "RTSAN_ENABLE";

    println!("cargo:rerun-if-env-changed={RTSAN_ENV_VAR}");

    // Hardcoded list of supported targets
    const SUPPORTED_TARGETS: [&str; 6] = [
        "x86_64-unknown-linux-gnu",
        "aarch64-unknown-linux-gnu",
        "x86_64-apple-darwin",
        "aarch64-apple-darwin",
        "aarch64-apple-ios",
        "x86_64-apple-ios",
    ];

    // Get the complete target triple
    let target = std::env::var("TARGET").unwrap_or_default();

    let is_supported = SUPPORTED_TARGETS.contains(&target.as_str());
    if is_supported {
        println!("cargo:rustc-cfg=rtsan_supported");
    }

    let is_enabled = match std::env::var(RTSAN_ENV_VAR).as_deref() {
        Ok("1") => true,
        Ok("0") | Err(std::env::VarError::NotPresent) => false,
        _ => panic!("RTSAN_ENABLE must be 0 (disabled) or 1 (enabled)"),
    };
    if is_enabled {
        if is_supported {
            println!("cargo:warning=RealtimeSanitizer enabled");
            println!("cargo:rustc-cfg=rtsan_enabled");
        } else {
            println!("cargo:error=Realtime Sanitizer not supported on target: {target}");
        }
    }
}
