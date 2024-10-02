use crate::modules::models::audio_model::AudioModel;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom}; // Import Seek and SeekFrom traits
use std::sync::{Arc, Mutex};

pub struct PlayerController {
    model: Arc<Mutex<AudioModel>>,
    stream: Option<Stream>,
    current_samples: Vec<f32>, // Buffer for the current audio samples
}

impl PlayerController {

    // Public constructor
    pub fn new(model: Arc<Mutex<AudioModel>>) -> Self {
        PlayerController {
            model,
            stream: None,
            current_samples: Vec::new(),
        }
    }

    pub fn play_current(&mut self) {
        // Lock the model to get the current file
        let current_file = {
            let model = self.model.lock().unwrap(); // Immutable borrow ends after this block
            model.get_current_file().to_path_buf() // Take ownership of the path to avoid borrowing issues
        };

        println!("Playing: {:?}", current_file.display());

        // Load the audio samples from the current file
        if let Err(err) = self.load_samples(&current_file) {
            // Use reference to the path
            eprintln!("Error loading samples: {:?}", err);
            return;
        }

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Failed to find output device");
        let config: StreamConfig = device.default_output_config().unwrap().config();

        // Create a new audio stream
        let samples = Arc::new(Mutex::new(self.current_samples.clone())); // Use cloned samples
        let samples_clone = Arc::clone(&samples);

        // Create the stream and ensure that the closure can safely access the samples
        self.stream = Some(
            device
                .build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        let samples = samples_clone.lock().unwrap(); // Mutable borrow for playback
                        let len = data.len().min(samples.len());
                        data[..len].copy_from_slice(&samples[..len]);
                        for sample in &mut data[len..] {
                            *sample = 0.0; // Fill remaining buffer with silence
                        }
                    },
                    move |err| {
                        eprintln!("Stream error: {:?}", err);
                    },
                    None, // User data
                )
                .unwrap(),
        );

        self.stream.as_ref().unwrap().play().unwrap();
    }

    // Load audio samples from a WAV file
    fn load_samples(&mut self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        // Read WAV header (for simplicity, we are not doing full WAV parsing)
        // This example assumes the WAV file has a valid header and is in PCM format.
        let mut cursor = Cursor::new(buffer);

        // Skip WAV header (44 bytes)
        cursor.seek(SeekFrom::Start(44))?; // Fixed: added Seek trait

        self.current_samples.clear(); // Clear previous samples

        // Read PCM samples (16-bit signed integers)
        let mut sample = [0u8; 2];
        while cursor.read_exact(&mut sample).is_ok() {
            let value = i16::from_le_bytes(sample);
            self.current_samples.push(value as f32 / i16::MAX as f32); // Normalize to -1.0 to 1.0
        }

        Ok(())
    }

    // pub fn stop(&mut self) {
    //     if let Some(stream) = &self.stream {
    //         stream.pause().unwrap();
    //     }
    //     println!("Playback paused.");
    // }

    pub fn next(&mut self) {
        self.model.lock().unwrap().next_track();
        self.play_current(); // Call play_current after track change
    }

    pub fn prev(&mut self) {
        self.model.lock().unwrap().prev_track();
        self.play_current(); // Call play_current after track change
    }
}
