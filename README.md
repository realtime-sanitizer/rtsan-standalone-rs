# rtsan-standalone-rs

This is a wrapper for the standalone version of RealtimeSanitizer (RTSan) to detect real-time violations in Rust applications.
You can find more information in the [Official Clang Docs](https://clang.llvm.org/docs/RealtimeSanitizer.html)
and the [RTSan Repository](https://github.com/realtime-sanitizer/rtsan).

> ⚠️ **Warning:** Currently, this sanitizer only works on Linux, macOS and iOS.

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

RTSan currently supports Linux, macOS and iOS.

To use RTSan, add it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
rtsan-standalone = "0.2.0"
```

To run your project with sanitizing enabled, execute:

```sh
RTSAN_ENABLE=1 cargo run
```

### Pre-built Libraries

By default this crate downloads pre-built libraries from the repo [rtsan-libs](https://github.com/realtime-sanitizer/rtsan-libs).

If you do not wish to use the pre-built libraries you can disable the default features and either let the build script build the library automatically
or provide a custom build of rtsan.

### Building locally

Ensure you have the following tools installed: `git`, `make`, and `cmake` (version 3.20.0 or higher).
Disable default features when adding `rtsan-standalone` to your project.
The initial build of `rtsan-standalone-sys` may take a few minutes to compile the LLVM
libraries. After building the crate for the first time, the library is located at:

```sh
target/debug/build/rtsan-standalone-sys-*/out/
```

### Using Custom-Built RTSan Libraries

To use a custom-built library, you need to set the `RTSAN_LIBRARY_PATH` environment variable.
When a library gets provided like this it will always be prioritized.

```sh
# Linux
RTSAN_LIBRARY_PATH=/path/to/libclang_rt.rtsan-x86_64.a RTSAN_ENABLE=1 cargo run
```

## Features

The `prebuilt-libs` feature enables automatic downloading of libraries from [rtsan-libs](https://github.com/realtime-sanitizer/rtsan-libs) and is activated by default, eliminating the need for local compilation.

## Examples

Explore the various possibilities with RTSan through the provided examples. For
instance, to run the [`vector`](examples/vector.rs) example, execute:

```sh
RTSAN_ENABLE=1 cargo run --example vector
```

## RTSan Options

You can set different options in RTSan like this:

```sh
RTSAN_OPTIONS=halt_on_error=false RTSAN_ENABLE=1 cargo run --example mutex
```

For a full list of options see here: [https://clang.llvm.org/docs/RealtimeSanitizer.html#run-time-flags](https://clang.llvm.org/docs/RealtimeSanitizer.html#run-time-flags).

## Contact

RTSan was invented by David Trevelyan and Ali Barker. The C++ upstream
implementation was authored by David Trevelyan and Chris Apple, while the Rust
wrapper was developed by Stephan Eckes. Feedback and contributions are welcome!

- **Discord**: [RealtimeSanitizer (RTSan)](https://discord.com/invite/DZqjbmSZzZ) Discord Channel
- **Email**: [realtime.sanitizer@gmail.com](mailto:realtime.sanitizer@gmail.com)
- **GitHub Issues**: Submit your queries or suggestions directly to this
  repository.
