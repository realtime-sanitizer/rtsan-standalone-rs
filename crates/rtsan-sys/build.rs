use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

fn main() {
    // Check for required tools
    check_tool("git");
    check_tool("cmake");
    check_tool("make");

    // Get target OS and architecture
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_os == "macos" {
        check_tool("install_name_tool");
    }

    // Set up paths
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

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
    run_command("make", &["-j8", "rtsan"], build_dir.to_str().unwrap());

    // Locate the built library
    let lib_path = if target_os == "linux" {
        build_dir.join(format!("lib/linux/libclang_rt.rtsan-{}.a", target_arch))
    } else {
        build_dir.join("lib/darwin/libclang_rt.rtsan_osx_dynamic.dylib")
    };

    if !lib_path.exists() {
        panic!("Built library not found at {:?}", lib_path);
    }

    // Copy the library to OUT_DIR
    let lib_name = lib_path.file_name().unwrap();
    let dest_lib_path = out_dir.join(lib_name);
    fs::copy(&lib_path, &dest_lib_path).expect("Failed to copy library to OUT_DIR");

    // Link the library
    println!("cargo:rustc-link-search=native={}", out_dir.display());

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
                dest_lib_path.to_str().unwrap(),
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
