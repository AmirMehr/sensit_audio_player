use hound; // Import the Hound library for reading WAV files.
use std::error::Error;
use std::path::Path;

use super::audio_loader::AudioFileLoader;

/// **WavLoader Struct**
///
/// This struct provides functionality to load WAV audio files and convert
/// them into a vector of samples (`Vec<f32>`) along with the sample rate.
///
/// Implements the `AudioFileLoader` trait to ensure a consistent interface.
pub struct WavLoader;

impl AudioFileLoader for WavLoader {
    /// Loads audio samples from a WAV file.
    ///
    /// This method reads the WAV file, extracts its samples, and converts them
    /// to floating-point values. It also retrieves the sample rate to ensure correct playback speed.
    ///
    /// # Parameters:
    /// - `path`: A reference to the path of the WAV file.
    ///
    /// # Returns:
    /// - `Ok((Vec<f32>, u32))`: A tuple containing the audio samples and the sample rate.
    /// - `Err(Box<dyn Error>)`: An error if the file cannot be opened or decoded.
    fn load_samples(&self, path: &Path) -> Result<(Vec<f32>, u32), Box<dyn Error>> {
        // Open the WAV file using the Hound library.
        let reader = hound::WavReader::open(path)?;

        // Retrieve the sample rate from the WAV file's specification.
        let sample_rate = reader.spec().sample_rate;

        // Convert the audio samples from i16 to f32 and normalize to [-1.0, 1.0].
        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .map(|s| s.unwrap() as f32 / i16::MAX as f32)
            .collect();

        // Return the samples along with the sample rate.
        Ok((samples, sample_rate))
    }
}
