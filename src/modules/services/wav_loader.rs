use hound;
use std::error::Error;
use std::path::Path;

use super::audio_loader::AudioFileLoader;

/// **WavLoader Struct**
///
/// This struct loads WAV files, extracting samples, sample rate, and channel count.
pub struct WavLoader;

impl AudioFileLoader for WavLoader {
    /// Loads WAV samples, sample rate, and channel count.
    ///
    /// # Parameters:
    /// - `path`: Path to the WAV file.
    ///
    /// # Returns:
    /// - `Ok((Vec<f32>, u32, u16))`: Samples, sample rate, and channels.
    /// - `Err(Box<dyn Error>)`: On failure, returns an error.
    fn load_samples(&self, path: &Path) -> Result<(Vec<f32>, u16), Box<dyn Error>> {
        let reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        let channels = spec.channels;

        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
            .collect();

        Ok((samples, channels))
    }
}
