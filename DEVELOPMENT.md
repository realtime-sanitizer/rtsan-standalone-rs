# Development

## Todo

- Add Github Actions CI for build / test / clippy / fmt / min version on Linux and macOS
- See if returns of scoped disabler are real-time safe so allocated vectors can be used afterwards
- nonblocking macro return values do not work for references (self.data, if data = &[f32])

## Benchmarking

Please always check that the functions get completely removed when the feature enable is not set, by running the benchmark without the feature and checking that every function call results in 0 ps execution time.
