use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Stream;
use std::error::Error;
use std::path::Path;

use super::mp3_loader::Mp3Loader;
use super::wav_loader::WavLoader;

/// **AudioLoader Trait**
///
/// This trait defines the behavior for creating an audio stream from a given audio file.
/// Implementations are responsible for decoding audio files and returning a `cpal::Stream` for playback.
pub trait AudioLoader {
    /// Creates an audio stream for playback.
    ///
    /// # Parameters:
    /// - `file_path`: The path to the audio file.
    ///
    /// # Returns:
    /// - `Ok(Stream)`: A `cpal::Stream` that can play the audio.
    /// - `Err(Box<dyn Error>)`: If loading the file or creating the stream fails.
    fn create_audio_stream(&self, file_path: &Path) -> Result<Stream, Box<dyn Error>>;
}

/// **AudioFileLoader Trait**
///
/// Provides functionality to load audio samples from a file into a `Vec<f32>` along with the sample rate.
/// This trait is useful for decoding audio files (like MP3 or WAV) into memory.
pub trait AudioFileLoader {
    /// Loads audio samples from the given file path.
    ///
    /// # Parameters:
    /// - `path`: The path to the audio file.
    ///
    /// # Returns:
    /// - `Ok((Vec<f32>, u32))`: On success, returns the decoded samples and sample rate.
    /// - `Err(Box<dyn Error>)`: On failure, returns an error.
    fn load_samples(&self, path: &Path) -> Result<(Vec<f32>, u32), Box<dyn Error>>;
}

/// **DynamicAudioLoader Struct**
///
/// This struct selects the appropriate loader (e.g., MP3 or WAV) based on the file extension
/// and creates a playback stream using `cpal`.
pub struct DynamicAudioLoader;

impl AudioLoader for DynamicAudioLoader {
    /// Creates an audio stream based on the file type (MP3 or WAV).
    ///
    /// This method selects the correct loader, loads the audio samples, and builds a `cpal::Stream`.
    ///
    /// # Parameters:
    /// - `file_path`: The path to the audio file.
    ///
    /// # Returns:
    /// - `Ok(Stream)`: A ready-to-use stream for playback.
    /// - `Err(Box<dyn Error>)`: If the file loading or stream creation fails.
    fn create_audio_stream(&self, file_path: &Path) -> Result<cpal::Stream, Box<dyn Error>> {
        // Get the default audio host on the system.
        let host = cpal::default_host();

        // Get the default output audio device.
        let device = host
            .default_output_device()
            .ok_or("No output device available")?;

        // Get the default configuration for the output device.
        let config = device.default_output_config()?;

        // Extract the file extension to select the appropriate loader.
        let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Load the samples and sample rate using the appropriate loader.
        let (samples, sample_rate) = match extension {
            "wav" => WavLoader.load_samples(file_path)?,
            "mp3" => Mp3Loader.load_samples(file_path)?,
            _ => return Err("Unsupported audio format".into()),
        };

        // Ensure the device sample rate matches the audio sample rate.
        if config.sample_rate().0 != sample_rate {
            return Err(format!(
                "Sample rate mismatch: device {} Hz, file {} Hz",
                config.sample_rate().0,
                sample_rate
            )
            .into());
        }

        // Get the sample format and channel count from the device configuration.
        let sample_format = config.sample_format();
        let channels = config.channels() as usize;

        // Define the error callback for handling stream errors.
        let err_fn = |err| eprintln!("An error occurred on the output stream: {}", err);

        // Track the current sample index during playback.
        let mut sample_index = 0;

        // Match the sample format and build the appropriate output stream.
        match sample_format {
            cpal::SampleFormat::F32 => device
                .build_output_stream(
                    &config.config(),
                    move |data: &mut [f32], _| {
                        // Process each frame of audio data.
                        for frame in data.chunks_mut(channels) {
                            if sample_index + channels <= samples.len() {
                                // Copy samples into the frame.
                                for (i, sample) in frame.iter_mut().enumerate() {
                                    *sample = samples[sample_index + i];
                                }
                                sample_index += channels;
                            } else {
                                // If out of samples, fill with silence.
                                for sample in frame.iter_mut() {
                                    *sample = 0.0;
                                }
                            }
                        }
                    },
                    err_fn, // Error callback.
                    None,   // No specific latency requested.
                )
                .map_err(|e| Box::new(e) as Box<dyn Error>),
            _ => Err("Unsupported sample format".into()), // Handle unsupported formats.
        }
    }
}
