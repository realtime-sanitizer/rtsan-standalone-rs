# rtsan-standalone-rs

This is a wrapper for the standalone version of RealtimeSanitizer (RTSan) to
detect real-time violations in Rust applications.

## Usage

Mark a real-time function with the `#[rtsan::non_blocking]` macro:

```rust
#[rtsan::non_blocking]
fn process(data: &mut [f32]) {
  let _ = vec![0.0; 16]; // oops!
}
```

At run-time, real-time violations are presented with a stack trace:

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
installed: `git`, `make`, and `cmake` (3.20.0 or higher).

To use RTSan, add it as a dependency:

```bash
cargo add rtsan --git https://github.com/realtime-sanitizer/rtsan-standalone-rs --branch dev
```

Alternatively, add it to your `Cargo.toml`:

```toml
[dependencies]
rtsan = { git = "https://github.com/realtime-sanitizer/rtsan-standalone-rs", branch = "dev" }
```

The initial build of `rtsan-sys` may take a few minutes to compile the LLVM
libraries.

We recommend using RTSan as an optional dependency behind a feature flag or as a
dev dependency to avoid shipping it in production builds. For an integration
example, refer to the
[integration-example README](examples/integration-example/README.md).

## Features

The `rtsan-std-types` feature is enabled by default and re-exports the entire
`std` library. This allows marking functions as blocking, which are not
currently detected by the sanitizer, such as `Mutex::lock`. Refer to the `mutex`
example or `integration-example` for further assistance.

## Examples

Explore various features of RTSan through the examples provided. For instance,
to run the `vector` example:

```bash
cargo run --example vector
```

To see how to integrate RTSan with feature flags, check the integration example
and run it with:

```bash
cargo run --package integration-example --features rtsan
```

## Contact

RTSan was invented by David Trevelyan and Ali Barker, the C++ upstream
implementation was authored by David Trevelyan and Chris Apple, and the Rust
wrapper by Stephan Eckes. Feedback and contributions are welcome!

- **Discord**: `RealtimeSanitizer (RTSan)` Discord Channel
- **Email**: [realtime.sanitizer@gmail.com](mailto:realtime.sanitizer@gmail.com)
- **GitHub Issues**: Submit your queries or suggestions directly in this
  repository.
