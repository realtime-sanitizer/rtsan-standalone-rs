# rtsan-sys

Unsafe Rust bindings for the RTSan standalone library.

## Supported Targets

Currently this crate only works on Linux and macOS.

## Generating Bindings

The Rust bindings are generated from the original C++ header file for the RTSan
standalone library. The header file is available at the following link:\
[RTSan Standalone Header](https://github.com/realtime-sanitizer/rtsan/blob/main/include/rtsan_standalone/rtsan_standalone.h)

To generate the bindings, use the following command:

```bash
bindgen rtsan_standalone.h -o rtsan_standalone.rs
```

### Installing Bindgen

You can install `bindgen` via `cargo` using the following command:

```bash
cargo install bindgen-cli
```
