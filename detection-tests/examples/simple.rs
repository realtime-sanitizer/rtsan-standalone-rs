// 

use rtsan_standalone::*;

fn main() {
    ensure_initialized();
    violation();
}

#[nonblocking]
fn violation() {
    let _ = Box::new(4);
}
