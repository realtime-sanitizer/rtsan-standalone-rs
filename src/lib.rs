pub use rtsan_macros::*;

pub use std::*;

pub mod sync;

pub fn realtime_enter() {
    unsafe { rtsan_sys::__rtsan_realtime_enter() };
}

pub fn realtime_exit() {
    unsafe { rtsan_sys::__rtsan_realtime_exit() };
}

pub fn enable() {
    unsafe { rtsan_sys::__rtsan_enable() };
}

pub fn disable() {
    unsafe { rtsan_sys::__rtsan_disable() };
}

pub fn ensure_initialized() {
    unsafe { rtsan_sys::__rtsan_ensure_initialized() };
}

pub fn notify_blocking_call(function_name: &'static str) {
    unsafe {
        rtsan_sys::__rtsan_notify_blocking_call(function_name.as_ptr() as *const std::ffi::c_char)
    };
}

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

    #[test]
    fn it_works() {
        crate::ensure_initialized();

        let mut my_vec = Vec::with_capacity(1);

        crate::realtime_enter();

        my_vec.push(1.0);

        crate::realtime_exit();
    }
}
