use std::sync::Arc;

use rtsan_standalone_rs::sync::Mutex;

pub struct State {
    value: usize,
}

#[rtsan_standalone_rs::non_blocking]
fn process(state: Arc<Mutex<State>>) {
    let mut guard = state.lock().unwrap();
    guard.value += 1;
}

fn main() {
    let state = Arc::new(Mutex::new(State { value: 0 }));
    process(state.clone());
}
