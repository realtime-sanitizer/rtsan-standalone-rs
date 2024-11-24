pub use rtsan_macros::*;

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

pub fn __rtsan_notify_blocking_call(blocking_function_name: &str) {
    let c_string = std::ffi::CString::new(blocking_function_name)
        .expect("String contained a null byte, which is not allowed in C strings.");
    unsafe { rtsan_standalone_sys::__rtsan_notify_blocking_call(c_string.as_ptr()) };
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
