// Intercepted call to real-time unsafe function `malloc` in real-time context!\n

use rtsan_standalone::*;

/// Add the [`rtsan::nonblocking`] macro to the process function.
/// In case the default-feature `sanitize` is not provided,
/// this macro won't do anything, so it can stay in production code.
#[nonblocking]
fn process(audio: &mut [f32]) {
    scoped_disabler! {
        assert_eq!(audio.len(), 256); // wrong assertions and panics will trigger the sanitizer before the panic message is printed!
    }
    let data = vec![2.0; 256]; // oops!
    for (output, input) in audio.iter_mut().zip(data) {
        *output *= input;
    }
}

fn main() {
    // call this always at the start of your program
    ensure_initialized();

    let mut audio = vec![1.0; 256];
    process(&mut audio);

    println!("Example finished without sanitizing!");
}
