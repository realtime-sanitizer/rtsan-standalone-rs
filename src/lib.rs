pub use rtsan_macros::*;

pub fn __rtsan_ensure_initialized() {
    unsafe { rtsan_standalone_sys::__rtsan_ensure_initialized() };
}

pub fn __rtsan_realtime_enter() {
    unsafe { rtsan_standalone_sys::__rtsan_realtime_enter() };
}

pub fn __rtsan_realtime_exit() {
    unsafe { rtsan_standalone_sys::__rtsan_realtime_exit() };
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
