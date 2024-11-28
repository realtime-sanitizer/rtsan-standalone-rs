//! This is a wrapper for the standalone version of RealtimeSanitizer (RTSan) to
//! detect real-time violations in Rust applications.
//!
//! ## Usage
//!
//! RTSan currently supports Linux and macOS. Ensure you have the following tools
//! installed: `git`, `make`, and `cmake` (3.20.0 or higher).
//!
//! To use RTSan, add it as a dependency:
//!
//! ```bash
//! cargo add rtsan --git https://github.com/realtime-sanitizer/rtsan-standalone-rs --branch dev
//! ```
//!
//! Alternatively, add it to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rtsan = { git = "https://github.com/realtime-sanitizer/rtsan-standalone-rs", branch = "dev" }
//! ```
//!
//! The initial build of `rtsan-sys` may take a few minutes to compile the LLVM
//! libraries.
//!
//! We recommend using RTSan as an optional dependency behind a feature flag or as a
//! dev dependency to avoid shipping it in production builds.
//!
//! ## Integration Example
//!
//! Update your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rtsan = { git = "https://github.com/realtime-sanitizer/rtsan-standalone-rs", branch = "dev", optional = true }
//!
//! [features]
//! rtsan = ["dep:rtsan"]
//!```
//! Conditionally use RTSan in your application:
//!
//! ```rust
//! #[cfg(feature = "rtsan")]
//! use rtsan as std;
//!
//! #[cfg_attr(feature = "rtsan", rtsan::non_blocking)]
//! pub fn process(&mut self, audio: &mut [f32]) { }
//! ```
//!
//! To detect locks in a Mutex currently the `rtsan` types have to be used. The
//! crate re-exports the `std` library for simplicity.
//!
//!
//! Run without RTSan:
//!
//! ```bash
//! cargo run --package integration-example
//! ```
//!
//! Expected output:
//!
//! ```
//! Example finished successfully!
//! ```
//!
//! Enable RTSan for detecting real-time violations:
//!
//! ```bash
//! cargo run --package integration-example --features rtsan
//! ```
//!
//! On detecting a violation, it produces an error like:
//!
//! ```bash
//! ==70107==ERROR: RealtimeSanitizer: blocking-call
//! Call to blocking function `lock` in real-time context!
//! ```
//!

#![allow(clippy::needless_doctest_main)]

#[cfg(feature = "rtsan-std-types")]
pub use std::*;

pub use rtsan_macros::*;

#[cfg(feature = "rtsan-std-types")]
pub mod sync;

/// Starts the sanitizer in your non-blocking function.
/// Use [`realtime_exit`] to end the sanitization scope.
/// The preferred usage is with the [`non_blocking`] macro.
///
/// # Example
///
/// ```
/// fn process() {
///     rtsan::realtime_enter();
///     let _ = [0.0; 256];
///     rtsan::realtime_exit();
/// }
///
/// // Preferred macro usage
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     let _ = [0.0; 256];
/// }
/// ```
pub fn realtime_enter() {
    unsafe { rtsan_sys::__rtsan_realtime_enter() };
}

/// Ends the sanitizer that was started with [`realtime_enter`].
/// The preferred usage is with the [`non_blocking`] macro.
///
/// # Example
///
/// ```
/// fn process() {
///     rtsan::realtime_enter();
///     let _ = [0.0; 256];
///     rtsan::realtime_exit();
/// }
///
/// // Preferred macro usage
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     let _ = [0.0; 256];
/// }
/// ```
pub fn realtime_exit() {
    unsafe { rtsan_sys::__rtsan_realtime_exit() };
}

/// Temporarily disables the sanitizer.
/// Re-enable it with [`enable`]. The preferred usage is with the
/// [`disabled_scope!`] macro for a small scope or with the [`no_sanitize`]
/// macro for a whole function.
///
/// # Example
///
/// ```
/// fn process() {
///     rtsan::realtime_enter();
///
///     rtsan::disable();
///     let mut data = Vec::with_capacity(1);
///     rtsan::enable();
///
///     data.push(0.0);
///
///     rtsan::realtime_exit();
/// }
///
/// // Preferred macro usage
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     let mut data = vec![];
///     rtsan::disabled_scope!({
///         data = Vec::with_capacity(1);
///     });
///     data.push(0.0);
/// }
/// ```
pub fn disable() {
    unsafe { rtsan_sys::__rtsan_disable() };
}

/// Re-enables the sanitizer after it has been disabled with [`disable`].
/// The preferred usage is with the [`disabled_scope`] macro.
///
/// # Example
///
/// ```
/// fn process() {
///     rtsan::realtime_enter();
///
///     rtsan::disable();
///     let mut data = Vec::with_capacity(1);
///     rtsan::enable();
///
///     data.push(0.0);
///
///     rtsan::realtime_exit();
/// }
///
/// // Preferred macro usage
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     let mut data = vec![];
///     rtsan::disabled_scope!({
///         data = Vec::with_capacity(1);
///     });
///     data.push(0.0);
/// }
/// ```
pub fn enable() {
    unsafe { rtsan_sys::__rtsan_enable() };
}

/// Initializes the realtime sanitizer. This function must be called as the first thing in your `main` function to ensure proper operation.
/// On some platforms, the sanitizer may work without explicitly calling this function, but calling it is recommended for guaranteed behavior.
///
/// # Example
///
/// ```
/// fn main() {
///     rtsan::ensure_initialized();
/// }
/// ```
pub fn ensure_initialized() {
    unsafe { rtsan_sys::__rtsan_ensure_initialized() };
}

/// Manually informs the sanitizer that the current function is blocking.
/// Provide the function name as a null-terminated string (e.g., `"my_function_name\0"`).
/// The preferred usage is with the [`blocking`] macro.
///
/// # Panics
///
/// Panics if the provided string is not null-terminated.
///
/// # Example
///
/// ```
/// fn my_blocking_function() {
///     rtsan::notify_blocking_call("my_blocking_function\0");
/// }
///
/// // Preferred macro usage
/// #[rtsan::blocking]
/// fn my_blocking_function_preferred() {}
/// ```
pub fn notify_blocking_call(function_name: &'static str) {
    if !function_name.ends_with('\0') {
        panic!("`rtsan::notify_blocking_call` requires a null-terminated function name (e.g., \"my_function_name\\0\").");
    }
    unsafe {
        rtsan_sys::__rtsan_notify_blocking_call(function_name.as_ptr() as *const std::ffi::c_char)
    };
}

/// Temporarily disables the sanitizer within a block of code.
/// This macro is the preferred way to handle temporary disabling of the sanitizer.
///
/// # Example
///
/// ```
/// #[rtsan::non_blocking]
/// fn process() {
///     let mut data = vec![];
///     rtsan::disabled_scope!({
///         data = Vec::with_capacity(1);
///     });
///     data.push(0.0);
/// }
/// ```
#[macro_export]
macro_rules! disabled_scope {
    ($block:block) => {{
        rtsan::disable();
        let result = (|| $block)();
        rtsan::enable();
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        notify_blocking_call("my_blocking_function\0");

        ensure_initialized();

        let mut my_vec = Vec::with_capacity(1);

        realtime_enter();

        my_vec.push(1.0);

        realtime_exit();
    }
}
