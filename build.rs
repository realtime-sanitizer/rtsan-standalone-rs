use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rtsan_enabled)");

    const RTSAN_ENV_VAR: &str = "RTSAN";

    println!("cargo:rerun-if-env-changed={}", RTSAN_ENV_VAR);

    // Get the complete target triple
    let target = std::env::var("TARGET").unwrap_or_default();

    if std::env::var(RTSAN_ENV_VAR).is_ok() {
        let targets_map = load_supported_targets()
            .expect("Could not load supported targets from `supported-targets.txt`.");

        let is_supported = targets_map.contains_key(&target);

        if is_supported {
            println!("cargo:warning={}", "RealtimeSanitizer enabled");
            println!("cargo:rustc-cfg=rtsan_enabled");
        } else {
            println!(
                "cargo:error={}",
                format!("Realtime Sanitizer not supported on target: {}", target)
            );
        }
    }
}

fn load_supported_targets() -> std::io::Result<std::collections::HashMap<String, String>> {
    use std::io::BufRead;
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cfg_path = manifest.join("supported-targets.txt");
    let file = fs::File::open(cfg_path)?;
    let reader = std::io::BufReader::new(file);
    let mut targets_map = std::collections::HashMap::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed_line = line.trim();

        // Skip empty lines and comments
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        // Split the line into key and value at the first '='
        if let Some((key, value)) = trimmed_line.split_once('=') {
            let target = key.trim().to_string();
            let filename = value.trim().to_string();
            if !target.is_empty() {
                targets_map.insert(target, filename);
            }
        }
    }
    Ok(targets_map)
}
