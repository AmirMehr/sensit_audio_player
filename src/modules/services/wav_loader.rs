use hound;
use std::error::Error;
use std::path::Path;

use super::audio_loader::AudioFileLoader;

pub struct WavLoader;

impl AudioFileLoader for WavLoader {
    fn load_samples(&self, path: &Path) -> Result<Vec<f32>, Box<dyn Error>> {
        let reader = hound::WavReader::open(path)?;
        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
            .collect();
        Ok(samples)
    }
}
