// This mutex lock will only be detected on macOS.
// The Linux mutex is spinning a couple of times before calling a system function,
// while the macOS mutex is always calling a system function.

use std::sync::{Arc, Mutex};

use rtsan_standalone::*;

pub struct State {
    value: usize,
}

// add the nonblocking macro to activate the sanitizer for this function
#[nonblocking]
fn process(state: Arc<Mutex<State>>) {
    let mut guard = state.lock().unwrap(); // oops!
    guard.value += 1;
}

fn main() {
    // call this always at the start of your program
    ensure_initialized();

    let state = Arc::new(Mutex::new(State { value: 0 })); // ok
    process(state.clone());
}
