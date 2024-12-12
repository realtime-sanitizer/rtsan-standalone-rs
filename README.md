# rtsan-standalone-rs

This is a wrapper for the standalone version of RealtimeSanitizer (RTSan) to
detect real-time violations in Rust applications.

## Todo
- Clarify if re-exporting std library is necessary
  - macOS Mutex is using pthread syscall and is detected
  - Linux is using not detected Futex but has a syscall in one specific case, when the lock can not be aquired fast
- Clarify if `sanitize` feature should have better name, or should be automatically enabled when building with debug profile.
- `rtsan::disabled_scope` macro should be called `scoped_disabler`
- See if returns of scoped disabler are real-time safe so allocated vectors can be used afterwards
- Detect number of cores in rtsan-sys build script instead of using fixed -j8

## Usage

Mark a real-time function with the `#[rtsan::nonblocking]` macro:

```rust
#[rtsan::nonblocking]
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

Currently, not all blocking functions in the standard library can be detected
(e.g., `Mutex::lock`). As a workaround, this library re-exports the standard
library and wraps some of its types to enable detection for more blocking
functions. To switch to the RTSan types, add the following to the top of your
file:

```rust
use rtsan::std;

use std::sync::Mutex;
```

Now you can use `std::sync::Mutex` and all other std types from rtsan. Just
beware, that when using `::std::sync::Mutex` the orginal Mutex will be used,
without sanitizing.

## Setup

RTSan currently supports Linux and macOS. Ensure you have the following tools
installed: `git`, `make`, and `cmake` (version 3.20.0 or higher).

To use RTSan, add it as a dependency in your `Cargo.toml` file and add the
`sanitize` feature to your project:

```toml
[dependencies]
rtsan = { git = "https://github.com/realtime-sanitizer/rtsan-standalone-rs", branch = "dev" }

[features]
sanitize = ["rtsan/sanitize"]
```

To run your project with sanitizing enabled, execute:

```sh
cargo run --features sanitize
```

The initial build of `rtsan-sys` may take a few minutes to compile the LLVM
libraries.

For more help, refer to the integration example
[README](examples/integration-example/README.md).

## Features

The `sanitize` feature allows you to enable or disable sanitizing for your
project. This ensures that all RTSan functions and macros can remain in your
production code without impacting performance when the feature is disabled.

## Examples

Explore the various possibilities with RTSan through the provided examples. For
instance, to run the [`vector`](examples/vector.rs) example, execute:

```sh
cargo run --example vector --features sanitize
```

The [integration example](examples/integration-example/) demonstrates how to
conditionally build the sanitizer into your project:

```sh
cargo run --package integration-example --features sanitize
```

All examples should fail with the `sanitize` feature enabled and work fine
without it.

## RTSan Options
You can set different options in RTSan like this:

```sh
RTSAN_OPTIONS=halt_on_error=false cargo run --example mutex --features sanitize
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
