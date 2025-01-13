use rtsan_standalone::*;

// Use the `nonblocking` macro to activate the real-time sanitizer for this function.
#[nonblocking]
fn my_function() {
    // Use the `scoped_disabler` macro to temporarily disable
    // the sanitizer in a real-time context.
    let my_data = scoped_disabler! {
        vec![0.0; 256]
    };

    // Panics and assertions trigger the allocator before printing the actual error.
    // Wrapping them in a `scoped_disabler` ensures the correct reason for the panic is shown.
    scoped_disabler! {
        assert_eq!(my_data.len(), 256);
    };

    let my_data2 = not_sanitized_function();

    // The implicit drop at the end of a scope triggers the sanitizer.
    // Explicitly dropping the allocated data prevents this.
    scoped_disabler! {
        drop(my_data);
        drop(my_data2);
    };
}

// Use the `no_sanitize_realtime` macro to disable the real-time sanitizer
// for the whole function.
#[no_sanitize_realtime]
fn not_sanitized_function() -> Vec<f64> {
    vec![0.0; 256]
}

fn main() {
    // Always call this function at the start of your program
    // to initialize the real-time sanitizer.
    ensure_initialized();

    my_function(); // Execute the sanitized function
}
