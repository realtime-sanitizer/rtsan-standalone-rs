pub use rtsan_macros::*;

pub mod sync;

pub fn __rtsan_realtime_enter() {
    unsafe { rtsan_standalone_sys::__rtsan_realtime_enter() };
}

pub fn __rtsan_realtime_exit() {
    unsafe { rtsan_standalone_sys::__rtsan_realtime_exit() };
}

pub fn __rtsan_enable() {
    unsafe { rtsan_standalone_sys::__rtsan_enable() };
}

pub fn __rtsan_disable() {
    unsafe { rtsan_standalone_sys::__rtsan_disable() };
}

pub fn __rtsan_ensure_initialized() {
    unsafe { rtsan_standalone_sys::__rtsan_ensure_initialized() };
}

pub fn __rtsan_notify_blocking_call(function_name: &'static str) {
    unsafe {
        rtsan_standalone_sys::__rtsan_notify_blocking_call(
            function_name.as_ptr() as *const std::ffi::c_char
        )
    };
}

#[macro_export]
macro_rules! disabled_scope {
    ($block:block) => {{
        unsafe { rtsan_standalone_sys::__rtsan_disable() };
        let result = (|| $block)();
        unsafe { rtsan_standalone_sys::__rtsan_enable() };
        result
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;

        __rtsan_ensure_initialized();

        let mut my_vec = Vec::with_capacity(1);

        __rtsan_realtime_enter();

        my_vec.push(1.0);

        __rtsan_realtime_exit();
    }
}
