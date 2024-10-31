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
pub trait AudioFileLoader {
    /// Loads audio samples, sample rate, and channel count from a file.
    ///
    /// # Parameters:
    /// - `path`: The path to the audio file.
    ///
    /// # Returns:
    /// - `Ok((Vec<f32>, u32, u16))`: On success, returns the decoded samples, sample rate, and channels.
    /// - `Err(Box<dyn Error>)`: On failure, returns an error.
    fn load_samples(&self, path: &Path) -> Result<(Vec<f32>, u32, u16), Box<dyn Error>>;
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
    fn create_audio_stream(&self, file_path: &Path) -> Result<Stream, Box<dyn Error>> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("No output device available")?;

        let config = device.default_output_config()?;
        let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        // Load the samples, sample rate, and channel count using the appropriate loader.
        let (samples, sample_rate, channels) = match extension {
            "wav" => WavLoader.load_samples(file_path)?,
            "mp3" => Mp3Loader.load_samples(file_path)?,
            _ => return Err("Unsupported audio format".into()),
        };

        let mut prefered_channels = channels;
        // Set prefered_channels to config.channels() if sample rates differ. Due to a few formats are not playing correctly in mp3 files.
        if config.sample_rate().0 != sample_rate {
            eprintln!(
                "[WARNING] Sample rate mismatch: device {} Hz, file {} Hz. Using the device default output config.",
                config.sample_rate().0,
                sample_rate
            );
            // In a more advenced programm we should add some logic to convert the rate here
            prefered_channels = config.channels();
        }

        // Error callback function for handling stream errors.
        let err_fn = |err| eprintln!("An error occurred on the output stream: {}", err);

        // Track the current sample index during playback.
        let mut sample_index = 0;

        match config.sample_format() {
            cpal::SampleFormat::F32 => device
                .build_output_stream(
                    &config.config(),
                    move |data: &mut [f32], _| {
                        for frame in data.chunks_mut(prefered_channels as usize) {
                            if sample_index + channels as usize <= samples.len() {
                                for (i, sample) in frame.iter_mut().enumerate() {
                                    *sample = samples[sample_index + i % channels as usize];
                                }
                                sample_index += channels as usize;
                            } else {
                                frame.fill(0.0);
                            }
                        }
                    },
                    err_fn,
                    None,
                )
                .map_err(|e| Box::new(e) as Box<dyn Error>),
            _ => Err("Unsupported sample format".into()),
        }
    }
}
