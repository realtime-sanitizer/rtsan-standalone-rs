use std::sync::Arc;

use rtsan::sync::Mutex;

pub struct State {
    value: usize,
}

#[rtsan::non_blocking]
fn process(state: Arc<Mutex<State>>) {
    let mut guard = state.lock().unwrap();
    guard.value += 1;
}

fn main() {
    rtsan::ensure_initialized();

    let state = Arc::new(Mutex::new(State { value: 0 }));
    process(state.clone());
}
