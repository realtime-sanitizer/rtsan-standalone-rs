# RTSan Integration Example

This project demonstrates how to integrate RTSan into your production code using
a feature flag.

## Setup

Update your `Cargo.toml` file as follows:

```toml
[dependencies]
rtsan-standalone = "0.1.0"

[features]
rtsan = ["rtsan-standalone/enable"]
```

With this setup, all RTSan macros and functions can remain in your production
code. By default, these functions will be empty definitions and will only work
when you activate the `enable` feature.

## Running the Example

### Running Without RTSan

To run the example without RTSan:

```sh
cargo run --package integration-example
```

Expected output:

```sh
Example finished without sanitizing!
```

### Running With RTSan Enabled

To enable RTSan and detect real-time violations, run the example with the
`rtsan` feature:

```sh
cargo run --package integration-example --features rtsan
```

If a real-time violation is detected, RTSan will produce an error like the
following:

```sh
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
