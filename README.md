# rtsan-standalone-rs

This is a Rust wrapper for RTSan to detect real-time violations in your
application.

## Prerequisites

This crate currently builds only on Linux and macOS. Ensure the following tools
are installed:

- git
- make
- cmake (version 3.20.0 or higher)

## Example Usage

The included example project illustrates the integration of RTSan in your
project.

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
violation is detected. In this case, it should produce the following error:

```
==70107==ERROR: RealtimeSanitizer: blocking-call
Call to blocking function `lock` in real-time context!
```

### Additional Examples

For more examples showcasing various features of RTSan, refer to the
[examples directory](crates/rtsan/examples).
