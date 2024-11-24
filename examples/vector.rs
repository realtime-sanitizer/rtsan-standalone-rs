#[rtsan::non_blocking]
fn my_function() {
    let _ = vec![0.0; 256];

    rtsan::disabled_scope!({
        let _ = vec![0.0; 256];
    });

    my_function2();
}

#[rtsan::no_sanitize]
fn my_function2() {
    let _ = vec![0.0; 256];
}

fn main() {
    rtsan::ensure_initialized();

    my_function();
}
