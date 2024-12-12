# Development

## Todo
- Add Github Actions CI for build / test / clippy / fmt / min version on Linux and macOS
- Clarify if re-exporting std library is necessary
  - macOS Mutex is using pthread syscall and is detected
  - Linux is using not detected Futex but has a syscall in one specific case, when the lock can not be aquired fast
- See if returns of scoped disabler are real-time safe so allocated vectors can be used afterwards
- Detect number of cores in rtsan-sys build script instead of using fixed -j8

## Benchmarking

Please always check that the functions get completely removed when the feature enable is not set, by running the benchmark without the feature and checking that every function call results in 0 ps execution time.