// For tracking mutex locks, currently the rtsan::sync::Mutex has to be used
#[cfg(feature = "rtsan")]
use rtsan::sync::Mutex;
use std::sync::Arc;
#[cfg(not(feature = "rtsan"))]
use std::sync::Mutex;

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
    /// Add the `rtsan::non_blocking` macro to the process function,
    /// in case the rtsan feature is activated
    #[cfg_attr(feature = "rtsan", rtsan::non_blocking)]
    pub fn process(&mut self, audio: &mut [f32]) {
        assert_eq!(audio.len(), 256);
        let guard = self.big_data.lock().unwrap();
        for (output, input) in audio.iter_mut().zip(*guard) {
            *output *= input;
        }
    }
}

fn main() {
    #[cfg(feature = "rtsan")]
    rtsan::ensure_initialized();

    let mut processor = MyProcessor::default();

    let mut audio = vec![1.0; 256];
    processor.process(&mut audio);

    println!("Example finished successfully!");
}
