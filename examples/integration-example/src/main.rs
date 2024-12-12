// Use the re-exported standard library from RTSan,
// allowing RTSan to report errors on standard functions
// that cannot otherwise be detected (e.g., `std::Mutex::lock`).
// This can remain enabled in production.
use rtsan::std;

use std::sync::{Arc, Mutex};

pub struct MyProcessor {
    big_data: Arc<Mutex<[f32; 256]>>,
}

impl Default for MyProcessor {
    fn default() -> Self {
        Self {
            big_data: Arc::new(Mutex::new([2.0; 256])),
        }
    }
}

impl MyProcessor {
    /// Add the [`rtsan::nonblocking`] macro to the process function.
    /// In case the default-feature `sanitize` is not provided,
    /// this macro won't do anything, so it can stay in production code.
    #[rtsan::nonblocking]
    pub fn process(&mut self, audio: &mut [f32]) {
        assert_eq!(audio.len(), 256); // wrong assertions and panics will trigger the sanitizer before the panic message is printed!
        let guard = self.big_data.lock().unwrap(); // oops !
        for (output, input) in audio.iter_mut().zip(*guard) {
            *output *= input;
        }
    }
}

fn main() {
    // call this always at the start of your program
    rtsan::ensure_initialized();

    let mut processor = MyProcessor::default();

    let mut audio = vec![1.0; 256];
    processor.process(&mut audio);

    println!("Example finished without sanitizing!");
}
