use rtsan_standalone_rs::{__rtsan_ensure_initialized, disabled_scope, no_sanitize, non_blocking};

#[non_blocking]
fn my_function() {
    let _ = vec![0.0; 256];

    disabled_scope!({
        let _ = vec![0.0; 256];
    });

    my_function2();
}

#[crate::no_sanitize]
fn my_function2() {
    let _ = vec![0.0; 256];
}

fn main() {
    __rtsan_ensure_initialized();

    my_function();
}
