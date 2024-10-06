extern crate cpal;

use std::path::Path;

// Define a trait for audio loading, adhering to Dependency Inversion Principle
pub trait AudioLoader {
    fn load_audio_samples(&self, path: &Path) -> Result<Vec<f32>, Box<dyn std::error::Error>>;
}
