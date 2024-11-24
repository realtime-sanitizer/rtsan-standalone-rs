# rtsan-standalone-sys

Unsafe Rust bindings for rtsan standalone library

## Generate bindings

Original C++ Header: https://github.com/realtime-sanitizer/rtsan/blob/main/include/rtsan_standalone/rtsan_standalone.h

```shell
bindgen rtsan_standalone.h -o rtsan_standalone.rs
```
Bindgen can be installed with

```shell
cargo install bindgen-cli
```