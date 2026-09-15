use rtsan_standalone::*;

// Export symbols so the optimizer cannot discard the functions being inspected.
#[no_mangle]
pub fn baseline(x: u64) -> u64 {
    x.wrapping_add(1)
}

#[no_mangle]
#[nonblocking]
pub fn my_nonblocking(x: u64) -> u64 {
    x.wrapping_add(1)
}

#[no_mangle]
#[blocking]
pub fn my_blocking(x: u64) -> u64 {
    x.wrapping_add(1)
}

#[no_mangle]
#[no_sanitize_realtime]
pub fn my_no_sanitize(x: u64) -> u64 {
    x.wrapping_add(1)
}

#[no_mangle]
#[nonblocking]
pub fn my_scoped_disabler(x: u64) -> u64 {
    scoped_disabler!({ x.wrapping_add(1) })
}

#[no_mangle]
pub fn empty_baseline() {}

#[no_mangle]
#[nonblocking]
pub fn empty_nonblocking() {}

#[no_mangle]
#[blocking]
pub fn empty_blocking() {}

#[no_mangle]
#[no_sanitize_realtime]
pub fn empty_no_sanitize() {}

#[no_mangle]
#[nonblocking]
pub fn empty_scoped_disabler() {
    scoped_disabler!({});
}
