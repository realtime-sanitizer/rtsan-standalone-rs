# rtsan-standalone-rs

This is a wrapper for the standalone version of RealtimeSanitizer (RTSan) to
detect real-time violations in Rust applications.

## Usage

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

We recommend using RTSan as an optional dependency behind a feature flag or as a
dev dependency to avoid shipping it in production builds. For an integration
example, refer to the
[integration example readme](examples/integration-example/README.md).

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

- **Discord**: RealtimeSanitize (RTSan) Discord Channel
- **Email**: [realtime.sanitizer@gmail.com](mailto:realtime.sanitizer@gmail.com)
- **GitHub Issues**: Submit your queries or suggestions directly in this
  repository.
