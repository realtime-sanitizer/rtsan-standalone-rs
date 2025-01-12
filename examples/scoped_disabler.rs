use rtsan_standalone::*;

// add the nonblocking macro to activate the sanitizer for this function
#[nonblocking]
fn my_function() {
    let _ = vec![0.0; 256]; // oops!

    // use the scoped_disabler macro to temporily disable
    // the sanitizer in a realtime context
    scoped_disabler!({
        let _ = vec![0.0; 256]; // ok
    });

    not_sanitized_function();
}

// add the `no_sanitize_realtime` macro to temporarily disable
// the sanitizer in a realtime context
#[no_sanitize_realtime]
fn not_sanitized_function() {
    let _ = vec![0.0; 256]; // ok
}

fn main() {
    // call this always at the start of your program
    ensure_initialized();

    my_function();
}
