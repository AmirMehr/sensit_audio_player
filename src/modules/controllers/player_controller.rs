extern crate cpal;

use crate::modules::models::audio_model::AudioModel;
use crate::modules::services::audio_loader::AudioLoader;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct PlayerController<L: AudioLoader> {
    model: Arc<Mutex<AudioModel>>,
    loader: L, // Use dependency injection for the loader
    stream: Option<Stream>,
    current_samples: Vec<f32>, // Buffer for the current audio samples
    is_paused: bool,           // Track if the player is paused
}

impl<L: AudioLoader> PlayerController<L> {
    pub fn new(model: Arc<Mutex<AudioModel>>, loader: L) -> Self {
        PlayerController {
            model,
            loader,
            stream: None,
            current_samples: Vec::new(),
            is_paused: true,
        }
    }

    pub fn load_current(&mut self) {
        // Step 1: Get the current file
        let current_file = self.get_current_file();

        // Step 2: Log the file name and start time
        println!("[LOG] Loading file: {:?}", current_file.display());
        let start_time = Instant::now(); // Start timing

        // Step 3: Load WAV samples from the file using the injected loader
        match self.loader.load_audio_samples(&current_file) {
            Ok(samples) => {
                self.current_samples = samples;
                // Log the time taken to load the file
                let duration = start_time.elapsed();
                println!("[LOG] Finished loading file in {:?}", duration);
            }
            Err(err) => {
                eprintln!("[ERROR] Error loading samples: {:?}", err);
                return;
            }
        }

        // Step 4: Initialize playback (CPAL)
        let (device, config) = match self.initialize_playback() {
            Ok((device, config)) => (device, config),
            Err(err) => {
                eprintln!("[ERROR] Failed to initialize playback: {:?}", err);
                return;
            }
        };

        // Step 5: Create audio stream
        if let Err(err) = self.create_audio_stream(&device, config) {
            eprintln!("[ERROR] Failed to create audio stream: {:?}", err);
            return;
        }
    }

    fn get_current_file(&self) -> std::path::PathBuf {
        let model = self.model.lock().unwrap();
        model.get_current_file().to_path_buf()
    }

    fn initialize_playback(
        &self,
    ) -> Result<(cpal::Device, StreamConfig), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or("[ERROR] Failed to find output device")?;
        let config = device.default_output_config()?.config();

        println!(
            "[LOG] Selected output device: {:?}",
            device.name().unwrap_or("Unknown device".to_string())
        );

        println!(
            "[LOG] CPAL configuration - Channels: {}, Sample rate: {}",
            config.channels, config.sample_rate.0
        );

        Ok((device, config))
    }

    fn create_audio_stream(
        &mut self,
        device: &cpal::Device,
        config: StreamConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let samples = Arc::new(Mutex::new(self.current_samples.clone())); // Use cloned samples
        let mut sample_pos = 0;

        self.stream = Some(device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let samples = samples.lock().unwrap();

                if sample_pos >= samples.len() {
                    println!("[LOG] Playback finished.");
                    for sample in data.iter_mut() {
                        *sample = 0.0; // Fill with silence
                    }
                    return;
                }

                let len = data.len().min(samples.len() - sample_pos);
                data[..len].copy_from_slice(&samples[sample_pos..sample_pos + len]);
                sample_pos += len;

                for sample in &mut data[len..] {
                    *sample = 0.0;
                }
            },
            move |err| {
                eprintln!("[ERROR] Stream error: {:?}", err);
            },
            None,
        )?);

        Ok(())
    }

    pub fn toggle_play(&mut self) {
        if let Some(stream) = &self.stream {
            if !self.is_paused {
                stream.pause().unwrap();
                self.is_paused = true;
                println!("[LOG] Playback paused.");
            } else {
                stream.play().unwrap();
                self.is_paused = false;
                println!("[LOG] Playback resumed.");
            }
        } else {
            println!("[ERROR] No active stream to toggle play/pause.");
        }
    }

    pub fn next(&mut self) {
        self.model.lock().unwrap().next_track();
        println!("[LOG] Playing next track...");
        self.load_current();
    }

    pub fn prev(&mut self) {
        self.model.lock().unwrap().prev_track();
        println!("[LOG] Playing previous track...");
        self.load_current();
    }
}
