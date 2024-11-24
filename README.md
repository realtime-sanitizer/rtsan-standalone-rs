# rtsan-standalone-rs
Rust crate repo for rtsan standalone

## Example

The example project shows how you would integrate rtsan into your application.

Run the example:

```sh
cargo run -p example
```

It will run fine and print `Example finished successfully!`.

If you activate the rtsan feature, rtsan will check the `process` function for real-time violations.

```sh
cargo run -p example -F rtsan
```

Now the application will crash with the following output

```sh
==70107==ERROR: RealtimeSanitizer: blocking-call
Call to blocking function `lock` in real-time context!
```
For more examples of every feature check `crates/rtsan/examples`.