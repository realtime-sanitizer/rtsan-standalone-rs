fn main() {
    let target = std::env::var("TARGET").expect("Could not find target");
    println!("Target triple: {}", target);
    // Specify the directory where the `.a` library is located
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={manifest_dir}/lib/{target}",);

    // Specify the library to link with (omit the `lib` prefix and `.a` extension)
    println!("cargo:rustc-link-lib=static=clang_rt.rtsan");

    // Rebuild if the library changes
    println!("cargo:rerun-if-changed=./lib/libclang_rt.rtsan.a");
}