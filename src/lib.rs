#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![allow(clippy::needless_doctest_main)]

pub use rtsan_standalone_macros::*;

/// Enter real-time context.
/// When in a real-time context, RTSan interceptors will error if realtime
/// violations are detected. Calls to this method are injected at the code
/// generation stage when RTSan is enabled.
/// Corresponds to a [`nonblocking`] macro.
///
/// # Example
///
/// ```
/// use rtsan_standalone::*;
///
/// fn process() {
///     realtime_enter();
///     let _ = vec![0.0; 256]; // oops!
///     realtime_exit();
/// }
///
/// // Macro usage preferred
/// #[nonblocking]
/// fn process_preferred() {
///     let _ = vec![0.0; 256]; // oops!
/// }
/// ```
#[inline]
pub fn realtime_enter() {
    #[cfg(rtsan_enabled)]
    unsafe {
        rtsan_standalone_sys::__rtsan_realtime_enter();
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
/// use rtsan_standalone::*;
///
/// fn process() {
///     realtime_enter();
///     let _ = vec![0.0; 256]; // oops!
///     realtime_exit();
/// }
///
/// // Macro usage preferred
/// #[nonblocking]
/// fn process_preferred() {
///     let _ = vec![0.0; 256]; // oops!
/// }
/// ```
#[inline]
pub fn realtime_exit() {
    #[cfg(rtsan_enabled)]
    unsafe {
        rtsan_standalone_sys::__rtsan_realtime_exit();
    }
}

/// Disable all RTSan error reporting in an otherwise real-time context.
/// Must be paired with a call to [`enable`].
/// Corresponds to a [`scoped_disabler`] or [`no_sanitize_realtime`] macro.
///
/// # Example
///
/// ```
/// use rtsan_standalone::*;
///
/// fn process() {
///     realtime_enter();
///
///     disable();
///     let mut data = vec![0.0; 16]; // ok
///     enable();
///
///     realtime_exit();
/// }
///
/// // Macro usage preferred
/// #[nonblocking]
/// fn process_preferred() {
///     scoped_disabler!({
///         let mut data = vec![0.0; 16]; // ok
///     });
/// }
#[inline]
pub fn disable() {
    #[cfg(rtsan_enabled)]
    unsafe {
        rtsan_standalone_sys::__rtsan_disable();
    }
}

/// Re-enable all RTSan error reporting.
/// Must follow a call to [`disable`].
/// Corresponds to a [`scoped_disabler`] or [`no_sanitize_realtime`] macro.
///
/// # Example
///
/// ```
/// use rtsan_standalone::*;
///
/// fn process() {
///     realtime_enter();
///
///     disable();
///     let mut data = vec![0.0; 16]; // ok
///     enable();
///
///     realtime_exit();
/// }
///
/// // Macro usage preferred
/// #[nonblocking]
/// fn process_preferred() {
///     scoped_disabler!({
///         let mut data = vec![0.0; 16]; // ok
///     });
/// }
#[inline]
pub fn enable() {
    #[cfg(rtsan_enabled)]
    unsafe {
        rtsan_standalone_sys::__rtsan_enable();
    }
}

/// Initializes rtsan if it has not been initialized yet.
/// Used by the RTSan runtime to ensure that rtsan is initialized before any
/// other rtsan functions are called.
///
/// # Example
///
/// ```
/// use rtsan_standalone::*;
///
/// fn main() {
///     ensure_initialized();
/// }
/// ```
pub fn ensure_initialized() {
    #[cfg(rtsan_enabled)]
    unsafe {
        rtsan_standalone_sys::__rtsan_ensure_initialized();
    }
}

/// Allows the user to specify a function as not-real-time-safe
/// Including this in the first line of a function definition is
/// analogous to marking a function  with the [`blocking`] macro.
///
/// # Example
///
/// ```
/// use rtsan_standalone::*;
///
/// fn my_blocking_function() {
///     notify_blocking_call(c"my_blocking_function");
/// }
///
/// // Preferred macro usage
/// #[blocking]
/// fn my_blocking_function_preferred() {}
/// ```
#[allow(unused_variables)]
pub fn notify_blocking_call(function_name: &'static core::ffi::CStr) {
    #[cfg(rtsan_enabled)]
    {
        unsafe {
            rtsan_standalone_sys::__rtsan_notify_blocking_call(function_name.as_ptr());
        }
    }
}

/// Disable all RTSan error reporting in an otherwise real-time context.
///
/// # Example
///
/// ```
/// use rtsan_standalone::*;
///
/// #[nonblocking]
/// fn process_preferred() {
///     scoped_disabler! {
///         let mut data = vec![0.0; 16]; // ok
///     };
/// }
/// ```
#[macro_export]
macro_rules! scoped_disabler {
    ($($body:tt)*) => {{
        let __guard = rtsan_standalone::ScopedDisabler::default();
        $($body)*
    }};
}

/// Enter real-time context for the lifetime of the object.
/// When in a real-time context, RTSan interceptors will error if realtime
/// violations are detected.
/// Corresponds to a [`nonblocking`] macro.
///
/// # Example
///
/// ```
/// use rtsan_standalone::*;
///
/// fn process() {
///     {
///         let _guard = ScopedSanitizeRealtime::default();
///         let _ = vec![0.0; 256]; // not ok
///     }
///     let _ = vec![0.0; 256]; // ok
/// }
pub struct ScopedSanitizeRealtime;

impl Default for ScopedSanitizeRealtime {
    fn default() -> Self {
        realtime_enter();
        Self
    }
}

impl Drop for ScopedSanitizeRealtime {
    fn drop(&mut self) {
        realtime_exit();
    }
}

/// Disable all RTSan error reporting in an otherwise real-time context,
/// for the lifetime of the object.
/// Corresponds to a [`scoped_disabler`] or [`no_sanitize_realtime`] macro.
///
/// # Example
///
/// ```
/// use rtsan_standalone::*;
///
/// #[nonblocking]
/// fn process() {
///     {
///         let _guard = ScopedDisabler::default();
///         let mut data = vec![0.0; 16]; // ok
///     }
///     let mut data = vec![0.0; 16]; // not ok
/// }
///
/// // Macro usage preferred
/// #[nonblocking]
/// fn process_preferred() {
///     scoped_disabler!({
///         let mut data = vec![0.0; 16]; // ok
///     });
///     let mut data = vec![0.0; 16]; // not ok
/// }
pub struct ScopedDisabler;

impl Default for ScopedDisabler {
    fn default() -> Self {
        disable();
        Self
    }
}

impl Drop for ScopedDisabler {
    fn drop(&mut self) {
        enable();
    }
}
