extern crate cpal;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use super::audio_loader::AudioLoader;

pub struct WavLoader;

impl AudioLoader for WavLoader {
    fn load_audio_samples(&self, path: &Path) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let path = path.to_path_buf(); // Clone path to move into the thread
        let samples_result = Arc::new(Mutex::new(None)); // Shared result container

        let samples_result_clone = Arc::clone(&samples_result); // Clone the Arc for the thread
        thread::spawn(move || {
            let mut file = File::open(path).unwrap(); // Load the file inside the thread

            // Read and skip the first 44 bytes (WAV header)
            let mut header = [0u8; 44];
            file.read_exact(&mut header).unwrap();

            // Parse the header information (channels, sample rate, etc.)
            let num_channels = u16::from_le_bytes([header[22], header[23]]);
            let sample_rate = u32::from_le_bytes([header[24], header[25], header[26], header[27]]);
            let bits_per_sample = u16::from_le_bytes([header[34], header[35]]);

            // Log header information
            println!(
                "[LOG] WAV file format: Channels: {}, Sample rate: {}, Bits per sample: {}",
                num_channels, sample_rate, bits_per_sample
            );

            // Read and convert audio samples to f32 format
            let mut current_samples = Vec::new();
            let mut sample_buffer = vec![0u8; 2]; // Assuming 16-bit samples

            while file.read_exact(&mut sample_buffer).is_ok() {
                let sample = i16::from_le_bytes([sample_buffer[0], sample_buffer[1]]);
                current_samples.push(sample as f32 / i16::MAX as f32); // Normalize to -1.0 to 1.0
            }

            // Store the result in the shared Arc
            *samples_result_clone.lock().unwrap() = Some(current_samples);

            // Log the number of samples loaded
            println!("[LOG] Loaded audio samples.");
        })
        .join()
        .unwrap(); // Join the thread to ensure it's completed

        // Retrieve the loaded samples from the shared Arc
        let loaded_samples = samples_result.lock().unwrap().take();

        match loaded_samples {
            Some(samples) => Ok(samples),
            None => Err("Failed to load audio samples".into()),
        }
    }
}
