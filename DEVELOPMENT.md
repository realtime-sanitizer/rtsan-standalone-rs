# Development

## Benchmarking

Please always check that the functions get completely removed when the feature enable is not set, by running the benchmark without the feature and checking that every function call results in 0 ps execution time.

## Testing

When the detection tests fail or you want to add a new one, run `RTSAN_ENABLE=1 cargo run -p detection-tests --bin YOUR_TEST_NAME`
to get the failure output of the test. Then pick one or more deterministic, but for this test characteristic phrases (for example including
function names used in the test). Add those to the very top of the test file, without any other lines in between them:
```
// check: PHRASE_1
// check: PHRASE_2
```

# Release

Before creating a release, check that everything can be published to crates.io.

1. If there were changes in `rtsan-standalone` or `rtsan-standalone-macros`:
  - Increase workspace version number
  - Test if `rtsan-standalone-sys` can be published `cargo publish -p rtsan-standalone-sys --dry-run`
  - Check if `rtsan-standalone-macros` can be pubslished with `cargo publish -p rtsan-standalone-macros --dry-run`
  - Set `rtsan-stanalone-macros` dependency version number in top-level `Cargo.toml` to the newest version
  - Set `rtsan-stanalone-sys` dependency version number in top-level `Cargo.toml` to the newest version
2. Check that the right version numbers are reflected in `README.md`.
3. Create a new release on the GitHub main branch with a tag that has the same version number as the workspace
4. Set local repository to the release tag and publish to crates.io
  - `cargo publish -p rtsan-standalone-sys` (if changed)
  - `cargo publish -p rtsan-standalone-macros`
  - `cargo publish -p rtsan-standalone`
