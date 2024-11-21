fn main() {
    // Specify the directory where the `.a` library is located
    println!("cargo:rustc-link-search=native=/home/stephan/Development/rtsan-standalone-rs/rtsan-standalone-sys/lib");

    // Specify the library to link with (omit the `lib` prefix and `.a` extension)
    println!("cargo:rustc-link-lib=static=clang_rt.rtsan");

    // Rebuild if the library changes
    println!("cargo:rerun-if-changed=./lib/libclang_rt.rtsan.a");
}
