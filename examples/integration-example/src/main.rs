use rtsan_standalone::*;

#[derive(Default)]
pub struct MyProcessor {}

impl MyProcessor {
    /// Add the [`rtsan::nonblocking`] macro to the process function.
    /// In case the default-feature `sanitize` is not provided,
    /// this macro won't do anything, so it can stay in production code.
    #[nonblocking]
    pub fn process(&mut self, audio: &mut [f32]) {
        assert_eq!(audio.len(), 256); // wrong assertions and panics will trigger the sanitizer before the panic message is printed!
        let data = vec![2.0; 256]; // oops!
        for (output, input) in audio.iter_mut().zip(data) {
            *output *= input;
        }
    }
}

fn main() {
    // call this always at the start of your program
    ensure_initialized();

    let mut processor = MyProcessor::default();

    let mut audio = vec![1.0; 256];
    processor.process(&mut audio);
}
