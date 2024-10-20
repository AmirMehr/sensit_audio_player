use minimp3::{Decoder, Error as Mp3Error, Frame};
use std::error::Error;
use std::fs::File;
use std::path::Path;

use super::audio_loader::AudioFileLoader;

pub struct Mp3Loader;

impl AudioFileLoader for Mp3Loader {
    fn load_samples(&self, path: &Path) -> Result<Vec<f32>, Box<dyn Error>> {
        let file = File::open(path)?;
        let mut decoder = Decoder::new(file);
        let mut samples = Vec::new();
        // https://github.com/germangb/minimp3-rs/issues/42
        while let Ok(Frame { data, .. }) = decoder.next_frame() {
            samples.extend(data.iter().map(|&s| s as f32 / i16::MAX as f32));
        }

        Ok(samples)
    }
}
