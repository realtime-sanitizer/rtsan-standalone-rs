# Development

## Zero-cost assembly check

Run the assembly check with Python 3 and the Rust toolchain on Linux x86-64 or
ARM64:

```sh
python3 tests/assembly/check.py
```

The check explicitly sets `RTSAN_ENABLE=0`, builds the library in release mode,
and compiles `tests/assembly/fixture.rs` as a separate optimized consumer. It
compares the instructions for `#[nonblocking]`, `#[blocking]`,
`#[no_sanitize_realtime]`, and `#[nonblocking]` with `scoped_disabler!` against
uninstrumented baselines. Each case has both an empty body and a calculation with
a runtime input and return value. Empty functions must reduce to a return;
calculation functions must match the baseline instructions. Compiler aliases for
identical functions are accepted; missing symbols and extra instructions fail.

To test the assembly checker itself:

```sh
python3 -m unittest discover -s tests/assembly -p 'test_*.py'
```

## Testing

When the detection tests fail or you want to add a new one, run
`RTSAN_ENABLE=1 cargo run -p detection-tests --bin YOUR_TEST_NAME`
to get the failure output of the test. Then pick one or more deterministic, but
for this test characteristic phrases (for example including function names used
in the test). Add those to the very top of the test file, without any other lines
in between them:

```
// check: PHRASE_1
// check: PHRASE_2
```

# Release

Before creating a release, check that everything can be published to crates.io.

1. Increase workspace version number and do not forget to:
   - Set `rtsan-stanalone-macros` dependency version number in top-level
     `Cargo.toml` to the newest version
   - Set `rtsan-stanalone-sys` dependency version number in top-level `Cargo.toml`
     to the newest version
2. Check that the right version numbers are reflected in `README.md`.
3. Test if release works with a dry run
   - `cargo publish --workspace --dry-run` (cargo version > 1.90.0)
4. Create a new release on the GitHub main branch with a tag that has the same
   version number as the workspace
5. Set local repository to the release tag and publish to crates.io
   - `cargo publish --workspace` (cargo version > 1.90.0)
