# rtsan-standalone-rs

Rust crate repository for the RTSan standalone library.

## Overview

This crate demonstrates how to integrate RTSan into your Rust application for
detecting real-time violations.

## Example Usage

The included example project illustrates the integration of RTSan.

### Running the Example

Run the example without the RTSan feature:

```bash
cargo run --package example
```

This will execute successfully and print:

```
Example finished successfully!
```

### Enabling the RTSan Feature

To enable RTSan for detecting real-time violations in the `process` function,
activate the `rtsan` feature:

```bash
cargo run --package example --features rtsan
```

With the `rtsan` feature enabled, the application will crash if a real-time
violation is detected. For example, it may produce the following error:

```
==70107==ERROR: RealtimeSanitizer: blocking-call
Call to blocking function `lock` in real-time context!
```

### Additional Examples

For more examples showcasing various features of RTSan, refer to the
[examples directory](crates/rtsan/examples).
