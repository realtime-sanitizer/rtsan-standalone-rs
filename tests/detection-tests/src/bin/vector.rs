// check: RealtimeSanitizer: unsafe-library-call
// check: Intercepted call to real-time unsafe function `malloc` in real-time context!
use rtsan_standalone::*;

#[nonblocking]
fn process() {
    let _ = vec![2.0; 256];
}

fn main() {
    // call this always at the start of your program
    ensure_initialized();
    process();
}
