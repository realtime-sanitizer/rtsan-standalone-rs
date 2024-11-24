# rtsan-standalone-rs
Rust crate repo for rtsan standalone

## Examples

To run an example call this from the root of the project:

```sh
cargo run --example vector
```

The expected output should be:

```
==104123==ERROR: RealtimeSanitizer: unsafe-library-call
Intercepted call to real-time unsafe function `calloc` in real-time context!
```
