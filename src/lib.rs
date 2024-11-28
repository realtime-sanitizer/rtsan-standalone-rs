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

pub use std::*;

pub use rtsan_macros::*;

#[cfg(feature = "sanitize")]
pub mod sync;

/// Enter real-time context.
/// When in a real-time context, RTSan interceptors will error if realtime
/// violations are detected. Calls to this method are injected at the code
/// generation stage when RTSan is enabled.
/// Corresponds to a [`non_blocking`] macro.
///
/// # Example
///
/// ```
/// fn process() {
///     rtsan::realtime_enter();
///     let _ = vec![0.0; 256]; // oops!
///     rtsan::realtime_exit();
/// }
///
/// // Macro usage preferred
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     let _ = vec![0.0; 256]; // oops!
/// }
/// ```
pub fn realtime_enter() {
    #[cfg(feature = "sanitize")]
    unsafe {
        rtsan_sys::__rtsan_realtime_enter();
    }
}

/// Exit the real-time context.
/// When not in a real-time context, RTSan interceptors will simply forward
/// intercepted method calls to the real methods.
/// Corresponds to a [`non_blocking`] macro.
///
/// # Example
///
/// ```
/// fn process() {
///     rtsan::realtime_enter();
///     let _ = vec![0.0; 256]; // oops!
///     rtsan::realtime_exit();
/// }
///
/// // Macro usage preferred
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     let _ = vec![0.0; 256]; // oops!
/// }
/// ```
pub fn realtime_exit() {
    #[cfg(feature = "sanitize")]
    unsafe {
        rtsan_sys::__rtsan_realtime_exit();
    }
}

/// Disable all RTSan error reporting in an otherwise real-time context.
/// Must be paired with a call to [`enable`].
/// Corresponds to a [`disabled_scope`] or [`no_sanitize`] macro.
///
/// # Example
///
/// ```
/// fn process() {
///     rtsan::realtime_enter();
///
///     rtsan::disable();
///     let mut data = vec![0.0; 16]; // ok
///     rtsan::enable();
///
///     rtsan::realtime_exit();
/// }
///
/// // Macro usage preferred
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     rtsan::disabled_scope!({
///         let mut data = vec![0.0; 16]; // ok
///     });
/// }
pub fn disable() {
    #[cfg(feature = "sanitize")]
    unsafe {
        rtsan_sys::__rtsan_disable();
    }
}

/// Re-enable all RTSan error reporting.
/// Must follow a call to [`disable`].
/// Corresponds to a [`disabled_scope`] or [`no_sanitize`] macro.
///
/// # Example
///
/// ```
/// fn process() {
///     rtsan::realtime_enter();
///
///     rtsan::disable();
///     let mut data = vec![0.0; 16]; // ok
///     rtsan::enable();
///
///     rtsan::realtime_exit();
/// }
///
/// // Macro usage preferred
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     rtsan::disabled_scope!({
///         let mut data = vec![0.0; 16]; // ok
///     });
/// }
pub fn enable() {
    #[cfg(feature = "sanitize")]
    unsafe {
        rtsan_sys::__rtsan_enable();
    }
}

/// Initializes rtsan if it has not been initialized yet.
/// Used by the RTSan runtime to ensure that rtsan is initialized before any
/// other rtsan functions are called.
///
/// # Example
///
/// ```
/// fn main() {
///     rtsan::ensure_initialized();
/// }
/// ```
pub fn ensure_initialized() {
    #[cfg(feature = "sanitize")]
    unsafe {
        rtsan_sys::__rtsan_ensure_initialized();
    }
}

/// Allows the user to specify a function as not-real-time-safe
/// Including this in the first line of a function definition is
/// analogous to marking a function  with the [`blocking`] macro.
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
#[allow(unused_variables)]
pub fn notify_blocking_call(function_name: &'static str) {
    #[cfg(feature = "sanitize")]
    {
        if !function_name.ends_with('\0') {
            panic!("`rtsan::notify_blocking_call` requires a null-terminated function name (e.g., \"my_function_name\\0\").");
        }
        unsafe {
            rtsan_sys::__rtsan_notify_blocking_call(
                function_name.as_ptr() as *const std::ffi::c_char
            );
        }
    }
}

/// Disable all RTSan error reporting in an otherwise real-time context.
///
/// # Example
///
/// ```
/// #[rtsan::non_blocking]
/// fn process_preferred() {
///     rtsan::disabled_scope!({
///         let mut data = vec![0.0; 16]; // ok
///     });
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
