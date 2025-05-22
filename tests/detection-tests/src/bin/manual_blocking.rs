// check: Call to blocking function `violation` in real-time context!
use rtsan_standalone::*;

#[nonblocking]
fn main() {
    ensure_initialized();
    violation();
}

#[blocking]
fn violation() {}
