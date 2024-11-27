# RTSan Integration Example

This project demonstrates integrating RTSan into your application using a
feature flag.

## Setup

Update your `Cargo.toml`:

```toml
[dependencies]
rtsan = { git = "https://github.com/realtime-sanitizer/rtsan-standalone-rs", branch = "dev", optional = true }

[features]
rtsan = ["dep:rtsan"]
```

## Usage

Conditionally use RTSan in your application:

```rust
#[cfg(feature = "rtsan")]
use rtsan::sync::Mutex;
#[cfg(not(feature = "rtsan"))]
use std::sync::Mutex;

#[cfg_attr(feature = "rtsan", rtsan::non_blocking)]
pub fn process(&mut self, audio: &mut [f32]) { }
```

## Running the Example

Run without RTSan:

```bash
cargo run --package integration-example
```

Expected output:

```
Example finished successfully!
```

Enable RTSan for detecting real-time violations:

```bash
cargo run --package integration-example --features rtsan
```

On detecting a violation, it produces an error like:

```
==70107==ERROR: RealtimeSanitizer: blocking-call
Call to blocking function `lock` in real-time context!
```