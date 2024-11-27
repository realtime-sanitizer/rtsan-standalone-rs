#[rtsan::blocking]
fn blocking_function() {}

#[rtsan::non_blocking]
fn realtime_function() {
    blocking_function();
}

fn main() {
    rtsan::ensure_initialized();

    realtime_function();
}
