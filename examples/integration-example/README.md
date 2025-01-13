# RTSan Integration Example

This project demonstrates how to integrate RTSan into your production code using
a feature flag.

## Setup

Update your `Cargo.toml` file as follows:

```toml
[dependencies]
rtsan = { git = "https://github.com/realtime-sanitizer/rtsan-standalone-rs", branch = "dev" }

[features]
rtsan = ["rtsan/enable"]
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
==70107==ERROR: RealtimeSanitizer: blocking-call
Call to blocking function `lock` in real-time context!
```
