#[rtsan::non_blocking]
fn my_function() {
    let _ = vec![0.0; 256];
}

fn main() {
    rtsan::ensure_initialized();

    my_function();
}
