# rtsan-standalone-rs

This is a wrapper for the standalone version of RealtimeSanitizer (RTSan) to
detect real-time violations in Rust applications.

## Usage

Mark a real-time function with the `#[nonblocking]` macro:

```rust
use rtsan_standalone::nonblocking;

#[nonblocking]
fn process(data: &mut [f32]) {
    let _ = vec![0.0; 16]; // oops!
}
```

At runtime, real-time violations are presented with a stack trace:

```bash
==283082==ERROR: RealtimeSanitizer: unsafe-library-call
Intercepted call to real-time unsafe function `calloc` in real-time context!
    #0 0x55c0c3be8cf2 in calloc /tmp/.tmp6Qb4u2/llvm-project/compiler-rt/lib/rtsan/rtsan_interceptors_posix.cpp:470:34
    #1 0x55c0c3be4e69 in alloc::alloc::alloc_zeroed::hf760e6484fdf32c8 /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/alloc.rs:170:14
    #2 0x55c0c3be4e69 in alloc::alloc::Global::alloc_impl::hc0e9b7c86f5cad5c /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/alloc.rs:181:43
    #3 0x55c0c3be56fb in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::allocate_zeroed::h8f75ff921b519af6 /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/alloc.rs:246:9
    ...
    #27 0x55c0c3be2ab4 in _start (target/debug/examples/vector+0x2ab4) (BuildId: adb992a7e560cd00ef533c9333d3c033fb4a7c42)
SUMMARY: RealtimeSanitizer: unsafe-library-call /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/alloc.rs:170:14 in alloc::alloc::alloc_zeroed::hf760e6484fdf32c8
```

## Setup

RTSan currently supports Linux and macOS. Ensure you have the following tools
installed: `git`, `make`, and `cmake` (version 3.20.0 or higher).

To use RTSan, add it as a dependency in your `Cargo.toml` file and add the
`sanitize` feature to your project:

```toml
[dependencies]
rtsan-standalone = "0.1.0"

[features]
rtsan = ["rtsan-standalone/enable"]
```

To run your project with sanitizing enabled, execute:

```sh
cargo run --features rtsan
```

The initial build of `rtsan-standalone-sys` may take a few minutes to compile the LLVM
libraries.

For more help, refer to the integration example
[README](examples/integration-example/README.md).

## Pre-built Library

To optimize compile times and avoid rebuilding rtsan for each project, you can use a pre-built library. This section explains how to set up and use the pre-built library.

### Library Location
After building the crate for the first time, the library is typically located at:
```sh
target/debug/build/rtsan-standlone-sys-*/out/
```

### Setting Up RTSAN_LIBRARY_PATH
To use the pre-built library, you need to set the `RTSAN_LIBRARY_PATH` environment variable. Here are three ways to do this:

1. **Direct Shell Command**
   ```sh
   # Linux
   RTSAN_LIBRARY_PATH=/path/to/libclang_rt.rtsan-x86_64.a cargo run --features enable

   # macOS
   RTSAN_LIBRARY_PATH=/path/to/libclang_rt.rtsan_osx_dynamic.dylib cargo run --features enable
   ```

2. **Cargo Configuration**
   Add the following to your `.cargo/config.toml`:
   ```toml
   [env]
   RTSAN_LIBRARY_PATH = "/path/to/libclang_rt.rtsan-x86_64.a"
   ```

3. **Shell Configuration**
   Add this line to your shell's configuration file (`.zshrc`, `.bashrc`, etc.):
   ```sh
   export RTSAN_LIBRARY_PATH="/path/to/libclang_rt.rtsan-x86_64.a"
   ```

## Features

The `enable` feature allows you to enable or disable sanitizing for your
project. This ensures that all RTSan functions and macros can remain in your
production code without impacting performance when the feature is disabled.

## Examples

Explore the various possibilities with RTSan through the provided examples. For
instance, to run the [`vector`](examples/vector.rs) example, execute:

```sh
cargo run --example vector --features enable
```

The [integration example](./examples/integration-example/) demonstrates how to
conditionally build the sanitizer into your project:

```sh
cargo run --package integration-example --features rtsan
```

## RTSan Options
You can set different options in RTSan like this:

```sh
RTSAN_OPTIONS=halt_on_error=false cargo run --example mutex --features enable
```
For a full list of options see here: https://clang.llvm.org/docs/RealtimeSanitizer.html#run-time-flags.

## Contact

RTSan was invented by David Trevelyan and Ali Barker. The C++ upstream
implementation was authored by David Trevelyan and Chris Apple, while the Rust
wrapper was developed by Stephan Eckes. Feedback and contributions are welcome!

- **Discord**: `RealtimeSanitizer (RTSan)` Discord Channel
- **Email**: [realtime.sanitizer@gmail.com](mailto:realtime.sanitizer@gmail.com)
- **GitHub Issues**: Submit your queries or suggestions directly to this
  repository.
