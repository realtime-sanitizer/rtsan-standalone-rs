// use this to tell rtsan that this function is blocking,
// even if it can not be detected by RTSan.
#[rtsan::blocking]
fn blocking_function() {}

// add the nonblocking macro to activate the sanitizer for this function
#[rtsan::nonblocking]
fn realtime_function() {
    blocking_function(); // oops!
}

fn main() {
    // call this always at the start of your program
    rtsan::ensure_initialized();

    realtime_function();

    println!("Example finished without sanitizing!");
}
