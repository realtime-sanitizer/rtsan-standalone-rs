use rtsan_standalone::*;

#[nonblocking]
fn create_array_function() {
    let _ = [0.0; 256];
}

#[test]
fn test_nonblocking() {
    // call this always at the start of your program
    ensure_initialized();

    create_array_function();
}

#[no_sanitize_realtime]
fn not_sanitized_function() -> Vec<f64> {
    vec![0.0; 256]
}

#[nonblocking]
fn scoped_disabler_function() {
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

#[test]
fn test_scroped_disabler() {
    ensure_initialized();

    scoped_disabler_function();
}

#[blocking]
fn blocking_function() {}

// add the nonblocking macro to activate the sanitizer for this function
#[nonblocking]
fn call_blocking_function() {
    scoped_disabler! {
        blocking_function();
    }
}

#[test]
fn test_blocking() {
    // call this always at the start of your program
    ensure_initialized();

    call_blocking_function();
}
