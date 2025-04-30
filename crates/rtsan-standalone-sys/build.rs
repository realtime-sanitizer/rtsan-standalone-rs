use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::tempdir;

const LLVM_VERSION: &str = "v20.1.1.1";

fn main() {
    println!("cargo::rustc-check-cfg=cfg(rtsan_enabled)");

    const RTSAN_ENV_VAR: &str = "RTSAN";
    println!("cargo:rerun-if-env-changed={}", RTSAN_ENV_VAR);

    let target = std::env::var("TARGET").unwrap_or_default();

    let targets_map = load_supported_targets()
        .expect("Could not load supported targets from `supported-targets.txt`.");

    let is_supported = targets_map.contains_key(&target);
    let is_enabled = std::env::var(RTSAN_ENV_VAR).is_ok();

    if !is_supported || !is_enabled {
        return;
    }

    // RTSAN is enabled and supported
    println!("cargo:rustc-cfg=rtsan_enabled");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Check if custom library path is provided
    if let Ok(custom_lib_path) = env::var("RTSAN_LIB_PATH") {
        let custom_lib_path = PathBuf::from(custom_lib_path);
        if !custom_lib_path.exists() {
            panic!(
                "Provided library path does not exist: {:?}",
                custom_lib_path
            );
        }

        let expected_extension = if target_os == "linux" { "a" } else { "dylib" };
        if !custom_lib_path
            .extension()
            .map_or(false, |ext| ext == expected_extension)
        {
            panic!("Invalid library extension for target OS");
        }

        // Copy the library to OUT_DIR
        let lib_name = custom_lib_path.file_name().unwrap();
        let dest_lib_path = out_dir.join(lib_name);
        fs::copy(&custom_lib_path, &dest_lib_path).expect("Failed to copy library to OUT_DIR");

        setup_linking(&dest_lib_path, &target_os);
        return;
    }

    // Check if pre-built libraries should be downloaded
    if cfg!(feature = "prebuilt-libs") {
        let base_url = format!(
            "https://github.com/realtime-sanitizer/rtsan-libs/releases/download/{}/",
            LLVM_VERSION
        );

        let filename = targets_map
            .get(&target)
            .expect("Target should be in map if is_supported was true");

        let url = base_url + filename;

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        let out_path = out_dir.join(&filename);

        // Download if not already present
        if !out_path.exists() {
            println!("Downloading {} to {:?}", url, out_path);
            let response = reqwest::blocking::get(&url)
                .expect("Failed to download file")
                .bytes()
                .expect("Failed to read bytes");
            fs::write(&out_path, &response).expect("Failed to write library file");
        }

        setup_linking(&out_path, &target_os);
        return;
    }

    // Build from source if no libraries were provided
    check_tool("git");
    check_tool("cmake");
    check_tool("make");

    if target_os == "macos" {
        check_tool("install_name_tool");
    }

    // Create a unique temporary directory
    let temp_dir = tempdir().expect("Failed to create temporary directory");
    let llvm_project_dir = temp_dir.path().join("llvm-project");

    // Clone llvm-project into the temporary directory
    run_command(
        "git",
        &[
            "clone",
            "-n",
            "--depth=1",
            "--filter=tree:0",
            "--branch",
            "llvmorg-20.1.0",
            "https://github.com/llvm/llvm-project.git",
            llvm_project_dir.to_str().unwrap(),
        ],
        ".",
    );

    // Perform sparse checkout
    run_command(
        "git",
        &[
            "sparse-checkout",
            "set",
            "--no-cone",
            "compiler-rt",
            "cmake",
        ],
        llvm_project_dir.to_str().unwrap(),
    );
    run_command("git", &["checkout"], llvm_project_dir.to_str().unwrap());

    // Build the library
    let build_dir = llvm_project_dir.join("build");
    if !build_dir.exists() {
        fs::create_dir(&build_dir).expect("Failed to create build directory");
    }
    run_command(
        "cmake",
        &[
            "-G",
            "Unix Makefiles",
            "-DCMAKE_BUILD_TYPE=Release",
            "-DCOMPILER_RT_BUILD_SANITIZERS=ON",
            "-DLLVM_TARGETS_TO_BUILD=Native",
            "../compiler-rt",
        ],
        build_dir.to_str().unwrap(),
    );
    let num_cores = num_cpus::get();
    run_command(
        "make",
        &[&format!("-j{}", num_cores), "rtsan"],
        build_dir.to_str().unwrap(),
    );

    let lib_path = if target_os == "linux" {
        build_dir.join(format!("lib/linux/libclang_rt.rtsan-{}.a", target_arch))
    } else {
        build_dir.join("lib/darwin/libclang_rt.rtsan_osx_dynamic.dylib")
    };

    if !lib_path.exists() {
        panic!("Built library not found at {:?}", lib_path);
    }

    let lib_name = lib_path.file_name().unwrap();
    let dest_lib_path = out_dir.join(lib_name);
    fs::copy(&lib_path, &dest_lib_path).expect("Failed to copy library to OUT_DIR");

    setup_linking(&dest_lib_path, &target_os);
}

fn load_supported_targets() -> std::io::Result<std::collections::HashMap<String, String>> {
    use std::io::BufRead;
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let cfg_path = manifest.join("../../supported-targets.txt");
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

fn setup_linking(lib_path: &Path, target_os: &str) {
    let out_dir = lib_path.parent().unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir.display());

    let lib_name = lib_path.file_name().unwrap();
    if target_os == "linux" {
        let lib_stem = lib_name
            .to_str()
            .unwrap()
            .trim_start_matches("lib")
            .trim_end_matches(".a");
        println!("cargo:rustc-link-lib=static={}", lib_stem);
    } else {
        // Adjust install_name for macOS
        run_command(
            "install_name_tool",
            &[
                "-id",
                "@rpath/libclang_rt.rtsan_osx_dynamic.dylib",
                lib_path.to_str().unwrap(),
            ],
            ".",
        );

        // Link the dylib
        println!(
            "cargo:rustc-link-lib=dylib={}",
            lib_name
                .to_str()
                .unwrap()
                .trim_start_matches("lib")
                .trim_end_matches(".dylib")
        );

        // Set rpath to OUT_DIR
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", out_dir.display());
    }
}

fn check_tool(tool: &str) {
    if Command::new(tool).arg("--version").output().is_err() {
        println!(
            "cargo:warning=Required tool '{}' not found in PATH. Please install it.",
            tool
        );
    }
}

fn run_command(cmd: &str, args: &[&str], dir: &str) {
    let status = Command::new(cmd)
        .args(args)
        .current_dir(dir)
        .status()
        .unwrap_or_else(|_| panic!("Failed to run '{}'", cmd));
    if !status.success() {
        panic!("Command '{}' failed with status {:?}", cmd, status);
    }
}
