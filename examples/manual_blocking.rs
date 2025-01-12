use rtsan_standalone::*;

// use this to tell rtsan that this function is blocking,
// even if it can not be detected by RTSan.
#[blocking]
fn blocking_function() {}

// add the nonblocking macro to activate the sanitizer for this function
#[nonblocking]
fn realtime_function() {
    blocking_function(); // oops!
}

fn main() {
    // call this always at the start of your program
    ensure_initialized();

    realtime_function();
}
