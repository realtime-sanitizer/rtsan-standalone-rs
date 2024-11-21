pub use rtsan_standalone_sys::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;

        unsafe { __rtsan_ensure_initialized() };

        let mut my_vec = Vec::with_capacity(1);

        unsafe { __rtsan_realtime_enter() };

        my_vec.push(1.0);

        unsafe { __rtsan_realtime_exit() };
    }
}
