// add the nonblocking macro to activate the sanitizer for this function
#[rtsan::nonblocking]
fn my_function() {
    let _ = vec![0.0; 256]; // oops!

    // use the disabled_scope macro to temporily disable
    // the sanitizer in a realtime context
    rtsan::disabled_scope!({
        let _ = vec![0.0; 256]; // ok
    });

    not_sanitized_function();
}

// add the no_sanitize macro to temporarily disable
// the sanitizer in a realtime context
#[rtsan::no_sanitize]
fn not_sanitized_function() {
    let _ = vec![0.0; 256]; // ok
}

fn main() {
    // call this always at the start of your program
    rtsan::ensure_initialized();

    my_function();

    println!("Example finished without sanitizing!");
}
