use rtsan_standalone_rs::{__rtsan_ensure_initialized, blocking, non_blocking};

#[blocking]
fn blocking_function() {}

#[non_blocking]
fn realtime_function() {
    blocking_function();
}

fn main() {
    __rtsan_ensure_initialized();

    realtime_function();
}
