use rtsan_standalone::*;

#[nonblocking]
fn main() {
    ensure_initialized();
    violation();
}

#[blocking]
fn violation() {}
