#[rtsan::non_blocking]
fn my_function() {
    let _ = [0.0; 256];

    rtsan::disabled_scope!({
        let _ = vec![0.0; 256];
    });

    not_sanitized_function();
}

#[rtsan::no_sanitize]
fn not_sanitized_function() {
    let _ = vec![0.0; 256];
}

fn main() {
    rtsan::ensure_initialized();

    my_function();
}
