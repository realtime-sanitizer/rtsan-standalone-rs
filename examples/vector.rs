use rtsan_standalone_rs::{__rtsan_ensure_initialized, non_blocking};

#[non_blocking]
fn my_function() {
    let _ = vec![0.0; 256];
}

fn main() {
    __rtsan_ensure_initialized();

    my_function();
}
