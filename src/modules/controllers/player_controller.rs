use crate::modules::models::audio_model::AudioModel;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use std::sync::{Arc, Mutex};
use hound;

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

        println!("Playing: {:?}", current_file.display());

        // Load the audio samples from the current WAV file
        if let Err(err) = self.load_wav_samples(&current_file) {
            eprintln!("Error loading samples: {:?}", err);
            return;
        }

        // Initialize CPAL for playback
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Failed to find output device");
        let config: StreamConfig = device.default_output_config().unwrap().config();

        let samples = Arc::new(Mutex::new(self.current_samples.clone())); // Use cloned samples
        let samples_clone = Arc::clone(&samples);

        // Create the audio stream
        self.stream = Some(
            device
                .build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        let samples = samples_clone.lock().unwrap();
                        let len = data.len().min(samples.len());
                        data[..len].copy_from_slice(&samples[..len]);
                        for sample in &mut data[len..] {
                            *sample = 0.0; // Fill remaining buffer with silence
                        }
                    },
                    move |err| {
                        eprintln!("Stream error: {:?}", err);
                    },
                    None, // No user data
                )
                .unwrap(),
        );

        self.stream.as_ref().unwrap().play().unwrap();
    }

    // Load audio samples from a WAV file using the hound library
    fn load_wav_samples(&mut self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = hound::WavReader::open(path)?;
        let spec = reader.spec();

        if spec.bits_per_sample != 16 || spec.sample_rate != 44100 {
            return Err("Only 16-bit PCM WAV files with 44.1kHz are supported".into());
        }

        self.current_samples.clear(); // Clear previous samples

        // Read WAV samples and convert them to f32 format
        for sample in reader.samples::<i16>() {
            let sample = sample?;
            self.current_samples.push(sample as f32 / i16::MAX as f32); // Normalize to -1.0 to 1.0
        }

        Ok(())
    }

    pub fn next(&mut self) {
        self.model.lock().unwrap().next_track();
        self.play_current();
    }

    pub fn prev(&mut self) {
        self.model.lock().unwrap().prev_track();
        self.play_current();
    }
}
