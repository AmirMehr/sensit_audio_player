extern crate hound;
extern crate cpal;

use crate::modules::models::audio_model::AudioModel;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{ Stream, StreamConfig};
use std::sync::{Arc, Mutex};

pub struct PlayerController {
    model: Arc<Mutex<AudioModel>>,
    stream: Option<Stream>,
    current_samples: Vec<f32>, // Buffer for the current audio samples
}

impl PlayerController {
    pub fn new(model: Arc<Mutex<AudioModel>>) -> Self {
        PlayerController {
            model,
            stream: None,
            current_samples: Vec::new(),
        }
    }

    pub fn play_current(&mut self) {
        let current_file = {
            let model = self.model.lock().unwrap();
            model.get_current_file().to_path_buf()
        };

        println!("Playing file: {:?}", current_file.display());

        // Log: Loading the file
        println!("[LOG] Loading WAV file...");

        // Load the audio samples from the current WAV file
        if let Err(err) = self.load_wav_samples(&current_file) {
            eprintln!("[ERROR] Error loading samples: {:?}", err);
            return;
        }

        // Log: WAV file successfully loaded
        println!("[LOG] Successfully loaded the WAV file.");

        // Initialize CPAL for playback
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("[ERROR] Failed to find output device");

        // Log: Output device selected
        println!("[LOG] Selected output device: {:?}", device.name().unwrap_or("Unknown device".to_lowercase()));

        let config: StreamConfig = device.default_output_config().unwrap().config();

        // Log: Configuration details
        println!("[LOG] CPAL configuration - Channels: {}, Sample rate: {}", config.channels, config.sample_rate.0);

        let samples = Arc::new(Mutex::new(self.current_samples.clone())); // Use cloned samples
        let samples_clone = Arc::clone(&samples);

        // Add a separate counter for sample position within the closure
        let mut sample_pos = 0;

        // Create the audio stream
        self.stream = Some(
            device
                .build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        let samples = samples_clone.lock().unwrap();

                        // Check if we have enough samples left
                        if sample_pos >= samples.len() {
                            // Log when playback ends
                            println!("[LOG] Playback finished.");
                            for sample in data.iter_mut() {
                                *sample = 0.0; // Fill with silence
                            }
                            return;
                        }

                        // Log: Playback processing
                        // println!("[LOG] Processing audio stream buffer");

                        // Copy the samples to the audio stream buffer
                        let len = data.len().min(samples.len() - sample_pos);
                        data[..len].copy_from_slice(&samples[sample_pos..sample_pos + len]);
                        sample_pos += len;

                        // Fill the rest with silence if needed
                        for sample in &mut data[len..] {
                            *sample = 0.0;
                        }

                        // Log the current playback position
                        // println!("[LOG] Sample position: {}/{}", sample_pos, samples.len());
                    },
                    move |err| {
                        eprintln!("[ERROR] Stream error: {:?}", err);
                    },
                    None, // No user data
                )
                .unwrap(),
        );

        // Log: Audio stream created
        println!("[LOG] CPAL audio stream created.");

        // Start playing
        self.stream.as_ref().unwrap().play().unwrap();

        // Log: Playback started
        println!("[LOG] Playback started.");
    }

    // Load audio samples from a WAV file using the hound library
    fn load_wav_samples(&mut self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        // Log: WAV file header details
        println!("[LOG] WAV file format: {:?}, Channels: {}, Sample rate: {}", spec.sample_format, spec.channels, spec.sample_rate);

        self.current_samples.clear(); // Clear previous samples

        // Read WAV samples and convert them to f32 format
        for sample in reader.samples::<i16>() {
            let sample = sample?;
            self.current_samples.push(sample as f32 / i16::MAX as f32); // Normalize to -1.0 to 1.0
        }

        // Log: Number of samples loaded
        println!("[LOG] Loaded {} audio samples.", self.current_samples.len());

        Ok(())
    }

    pub fn next(&mut self) {
        self.model.lock().unwrap().next_track();
        println!("[LOG] Playing next track...");
        // self.play_current();
    }

    pub fn prev(&mut self) {
        self.model.lock().unwrap().prev_track();
        println!("[LOG] Playing previous track...");
        // self.play_current();
    }
}
