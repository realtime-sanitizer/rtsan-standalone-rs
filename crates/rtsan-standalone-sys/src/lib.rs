#![no_std]

#[cfg(rtsan_enabled)]
extern "C" {
    pub fn __rtsan_realtime_enter();
    pub fn __rtsan_realtime_exit();
    pub fn __rtsan_disable();
    pub fn __rtsan_enable();
    pub fn __rtsan_ensure_initialized();
    pub fn __rtsan_notify_blocking_call(blocking_function_name: *const core::ffi::c_char);
}
