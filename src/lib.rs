//! This is a wrapper for the standalone version of RealtimeSanitizer (RTSan) to
//! detect real-time violations in Rust applications.
//!
//! ## Usage
//!
//! Mark a real-time function with the `#[rtsan::nonblocking]` macro:
//!
//! ```rust
//! #[rtsan::nonblocking]
//! fn process(data: &mut [f32]) {
//!     let _ = vec![0.0; 16]; // oops!
//! }
//! ```
//!
//! At runtime, real-time violations are presented with a stack trace:
//!
//! ```bash
//! ==283082==ERROR: RealtimeSanitizer: unsafe-library-call
//! Intercepted call to real-time unsafe function `calloc` in real-time context!
//!     #0 0x55c0c3be8cf2 in calloc /tmp/.tmp6Qb4u2/llvm-project/compiler-rt/lib/rtsan/rtsan_interceptors_posix.cpp:470:34
//!     #1 0x55c0c3be4e69 in alloc::alloc::alloc_zeroed::hf760e6484fdf32c8 /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/alloc.rs:170:14
//!     #2 0x55c0c3be4e69 in alloc::alloc::Global::alloc_impl::hc0e9b7c86f5cad5c /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/alloc.rs:181:43
//!     #3 0x55c0c3be56fb in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::allocate_zeroed::h8f75ff921b519af6 /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/alloc.rs:246:9
//!     ...
//!     #27 0x55c0c3be2ab4 in _start (target/debug/examples/vector+0x2ab4) (BuildId: adb992a7e560cd00ef533c9333d3c033fb4a7c42)    
//! SUMMARY: RealtimeSanitizer: unsafe-library-call /rustc/f6e511eec7342f59a25f7c0534f1dbea00d01b14/library/alloc/src/alloc.rs:170:14 in alloc::alloc::alloc_zeroed::hf760e6484fdf32c8
//! ```
//!
//! Currently, not all blocking functions in the standard library can be detected
//! (e.g., `Mutex::lock`). As a workaround, this library re-exports the standard
//! library and wraps some of its types to enable detection for more blocking
//! functions. To switch to the RTSan types, add the following to the top of your
//! file:
//!
//! ```rust
//! use rtsan as std;
//!
//! use std::sync::Mutex;
//! ```
//!
//! Now you can use `std::sync::Mutex` and all other std types from rtsan. Just
//! beware, that when using `::std::sync::Mutex` the orginal Mutex will be used,
//! without sanitizing.
//!
//! ## Setup
//!
//! RTSan currently supports Linux and macOS. Ensure you have the following tools
//! installed: `git`, `make`, and `cmake` (version 3.20.0 or higher).
//!
//! To use RTSan, add it as a dependency in your `Cargo.toml` file and add the
//! `sanitize` feature to your project:
//!
//! ```toml
//! [dependencies]
//! rtsan = { git = "https://github.com/realtime-sanitizer/rtsan-standalone-rs", branch = "dev" }
//!
//! [features]
//! sanitize = ["rtsan/sanitize"]
//! ```
//!
//! To run your project with sanitizing enabled, execute:
//!
//! ```sh
//! cargo run --features sanitize
//! ```
//!
//! The initial build of `rtsan-sys` may take a few minutes to compile the LLVM
//! libraries.
//!
//! For more help, refer to the integration example
//! [README](examples/integration-example/README.md).
//!
//! ## Features
//!
//! The `sanitize` feature allows you to enable or disable sanitizing for your
//! project. This ensures that all RTSan functions and macros can remain in your
//! production code without impacting performance when the feature is disabled.
//!
//! ## Examples
//!
//! Explore the various possibilities with RTSan through the provided examples. For
//! instance, to run the [`vector`](examples/vector.rs) example, execute:
//!
//! ```sh
//! cargo run --example vector --features sanitize
//! ```
//!
//! The [integration example](examples/integration-example/) demonstrates how to
//! conditionally build the sanitizer into your project:
//!
//! ```sh
//! cargo run --package integration-example --features sanitize
//! ```
//!
//! All examples should fail with the `sanitize` feature enabled and work fine
//! without it.

#![allow(clippy::needless_doctest_main)]

pub use std::*;

pub use rtsan_macros::*;

#[cfg(feature = "sanitize")]
pub mod sync;

/// Enter real-time context.
/// When in a real-time context, RTSan interceptors will error if realtime
/// violations are detected. Calls to this method are injected at the code
/// generation stage when RTSan is enabled.
/// Corresponds to a [`nonblocking`] macro.
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
/// #[rtsan::nonblocking]
/// fn process_preferred() {
///     let _ = vec![0.0; 256]; // oops!
/// }
/// ```
#[inline]
pub fn realtime_enter() {
    #[cfg(feature = "sanitize")]
    unsafe {
        rtsan_sys::__rtsan_realtime_enter();
    }
}

/// Exit the real-time context.
/// When not in a real-time context, RTSan interceptors will simply forward
/// intercepted method calls to the real methods.
/// Corresponds to a [`nonblocking`] macro.
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
/// #[rtsan::nonblocking]
/// fn process_preferred() {
///     let _ = vec![0.0; 256]; // oops!
/// }
/// ```
#[inline]
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
/// #[rtsan::nonblocking]
/// fn process_preferred() {
///     rtsan::disabled_scope!({
///         let mut data = vec![0.0; 16]; // ok
///     });
/// }
#[inline]
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
/// #[rtsan::nonblocking]
/// fn process_preferred() {
///     rtsan::disabled_scope!({
///         let mut data = vec![0.0; 16]; // ok
///     });
/// }
#[inline]
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
/// #[rtsan::nonblocking]
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
