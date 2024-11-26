fn main() {
    let target = std::env::var("TARGET").expect("Could not find target");
    println!("Target triple: {}", target);
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    // Detect the operating system
    if cfg!(target_os = "macos") {
        create_symlink_macos();
        println!("cargo:rustc-link-search=native={manifest_dir}/lib/{target}",);
        println!("cargo:rustc-link-lib=dylib=clang_rt.rtsan_osx_dynamic");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-search=native={manifest_dir}/lib/{target}",);
        println!("cargo:rustc-link-lib=static=clang_rt.rtsan");
    } else {
        panic!("You are running on an unsupported operating system.");
    }

    // rerun build script
    println!(
        "cargo:rerun-if-changed={manifest_dir}/lib/{target}/libclang_rt.rtsan_osx_dynamic.dylib"
    );
    println!("cargo:rerun-if-changed={manifest_dir}/lib/{target}/libclang_rt.rtsan.a");
}

fn create_symlink_macos() {
    // Get the home directory
    let home_dir = std::env::var("HOME").expect("Could not get HOME directory");
    let lib_dir = std::path::PathBuf::from(home_dir).join("lib");

    // Create ~/lib/ if it doesn't exist
    if !lib_dir.exists() {
        std::fs::create_dir_all(&lib_dir).expect("Failed to create ~/lib directory");
    }

    // Path to your dynamic library
    let manifest_dir =
        std::env::var("CARGO_MANIFEST_DIR").expect("Could not get CARGO_MANIFEST_DIR");
    let lib_source = std::path::PathBuf::from(manifest_dir)
        .join("lib/aarch64-apple-darwin/libclang_rt.rtsan_osx_dynamic.dylib");

    // Path to symlink in ~/lib/
    let lib_symlink = lib_dir.join("libclang_rt.rtsan_osx_dynamic.dylib");

    // Remove the existing symlink if it exists
    if lib_symlink.exists() {
        std::fs::remove_file(&lib_symlink).expect("Failed to remove existing symlink");
    }

    // Create the symlink
    std::os::unix::fs::symlink(&lib_source, &lib_symlink)
        .expect("Failed to create symlink in ~/lib");
}
