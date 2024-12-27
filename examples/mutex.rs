use std::sync::{Arc, Mutex};

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
