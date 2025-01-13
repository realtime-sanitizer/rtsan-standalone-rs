use rtsan_standalone::*;
use std::sync::{Arc, Mutex};

// Define a shared state structure
pub struct State {
    value: usize,
}

// Use the `#[nonblocking]` macro to enable the sanitizer for this function.
#[nonblocking]
fn process(state: Arc<Mutex<State>>) {
    // Attempt to acquire the lock on the shared state.
    // This lock triggers a syscall because another thread is holding the lock at the same time.
    let mut guard = state.lock().unwrap();
    // Safely increment the shared state's value while holding the lock.
    guard.value += 1;
}

fn main() {
    // Always call this at the start of your program to ensure the sanitizer is initialized.
    ensure_initialized();

    // Create a shared state wrapped in a Mutex to ensure safe concurrent access.
    let state = Arc::new(Mutex::new(State { value: 0 }));

    // Acquire the lock in the current thread.
    let mut guard = state.lock().unwrap();

    // Clone the Arc to pass the shared state into a new thread.
    let state_clone = state.clone();

    // Spawn a new thread that will also attempt to acquire the lock.
    let handle = std::thread::spawn(|| {
        // Call the `process` function in the new thread.
        process(state_clone);
    });

    // Hold the lock and simulate work by sleeping for 1 second.
    // This ensures the other thread needs to wait for the lock, triggering a syscall.
    std::thread::sleep(std::time::Duration::from_secs(1));
    // Increment the value in the shared state while still holding the lock.
    guard.value += 1;

    // Explicitly release the lock by dropping the guard, allowing the other thread to proceed.
    drop(guard);

    // Wait for the other thread to finish execution.
    handle.join().unwrap();

    // Verify that both threads successfully incremented the shared state's value.
    assert_eq!(state.lock().unwrap().value, 2);
}
