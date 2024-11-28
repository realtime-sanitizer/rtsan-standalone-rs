// add the nonblocking macro to activate the sanitizer for this function
#[rtsan::non_blocking]
fn my_function() {
    let _ = vec![0.0; 256]; // oops
}

fn main() {
    // call this always at the start of your program
    rtsan::ensure_initialized();

    my_function();
}
