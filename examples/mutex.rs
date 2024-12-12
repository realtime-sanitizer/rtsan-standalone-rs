// Use the re-exported standard library from RTSan,
// allowing RTSan to report errors on standard functions
// that cannot otherwise be detected (e.g., `std::Mutex::lock`).
// This can remain enabled in production.
use rtsan::std;

// this is now using rtsan::sync::{Arc, Mutex}
use std::sync::{Arc, Mutex};

// don't use this!
// ::std::sync::Mutex
// -- this is using the normal std library

pub struct State {
    value: usize,
}

// add the nonblocking macro to activate the sanitizer for this function
#[rtsan::nonblocking]
fn process(state: Arc<Mutex<State>>) {
    let mut guard = state.lock().unwrap(); // oops!
    guard.value += 1;
}

fn main() {
    // call this always at the start of your program
    rtsan::ensure_initialized();

    let state = Arc::new(Mutex::new(State { value: 0 })); // ok
    process(state.clone());

    println!("Example finished without sanitizing!");
}
