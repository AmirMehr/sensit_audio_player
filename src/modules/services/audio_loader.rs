use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Stream;
use std::error::Error;
use std::path::Path;

use super::mp3_loader::Mp3Loader;
use super::wav_loader::WavLoader;

pub trait AudioLoader {
    fn create_audio_stream(&self, file_path: &Path) -> Result<Stream, Box<dyn Error>>;
}

pub trait AudioFileLoader {
    fn load_samples(&self, path: &Path) -> Result<Vec<f32>, Box<dyn Error>>;
}

pub struct DynamicAudioLoader;

impl AudioLoader for DynamicAudioLoader {
    // Public method to create an audio stream
    fn create_audio_stream(&self, file_path: &Path) -> Result<cpal::Stream, Box<dyn Error>> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("No output device available")?;
        let config = device.default_output_config()?;

        let extension = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let samples = match extension {
            "wav" => WavLoader.load_samples(file_path)?,
            "mp3" => Mp3Loader.load_samples(file_path)?,
            _ => return Err("Unsupported audio format".into()),
        };

        let sample_format = config.sample_format();
        let channels = config.channels() as usize;
        let err_fn = |err| eprintln!("An error occurred on the output stream: {}", err);

        let mut sample_index = 0;

        match sample_format {
            cpal::SampleFormat::F32 => device
                .build_output_stream(
                    &config.config(),
                    move |data: &mut [f32], _| {
                        for frame in data.chunks_mut(channels) {
                            if sample_index < samples.len() {
                                for (i, sample) in frame.iter_mut().enumerate() {
                                    *sample = samples[sample_index + i];
                                }
                                sample_index += channels;
                            } else {
                                for sample in frame.iter_mut() {
                                    *sample = 0.0;
                                }
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
