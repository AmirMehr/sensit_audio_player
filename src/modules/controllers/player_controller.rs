extern crate cpal;

use crate::modules::models::audio_folder_model::AudioFolderModel;
use crate::modules::services::audio_loader::AudioLoader;
use cpal::traits::StreamTrait;
use cpal::Stream;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub struct PlayerController {
    audio_model: Arc<Mutex<AudioFolderModel>>,
    audio_loader: Box<dyn AudioLoader>, // Boxed trait object for dynamic dispatch
    stream: Option<Stream>,
    is_playing: bool,
}

impl PlayerController {
    pub fn new(
        audio_model: Arc<Mutex<AudioFolderModel>>,
        audio_loader: Box<dyn AudioLoader>,
    ) -> Self {
        PlayerController {
            audio_model,
            audio_loader,
            stream: None,
            is_playing: false,
        }
    }

    pub fn load_current(&mut self) {
        let current_file = self.get_current_file();
        println!("[SENSIT_LOG] Loading file: {:?}", current_file.display());

        let start_time = Instant::now();

        // Step 1: Use the audio loader to create a stream for the current file
        match self.audio_loader.create_audio_stream(&current_file) {
            Ok(stream) => {
                self.stream = Some(stream);
                println!(
                    "[SENSIT_LOG] Stream created successfully for file {:?} in {:?}.",
                    current_file.display(),
                    start_time.elapsed()
                );

                self.play(); // Start playing the stream
            }
            Err(err) => {
                eprintln!("[ERROR] Failed to create stream: {:?}", err);
            }
        }
    }

    fn get_current_file(&self) -> PathBuf {
        let audio_model = self.audio_model.lock().unwrap();
        audio_model.get_current_file().to_path_buf()
    }

    pub fn toggle_play(&mut self) {
        if self.is_playing {
            self.stop(); // Stop to simulate pause
        } else {
            self.play(); // Play the stream
        }
    }

    pub fn play(&mut self) {
        if let Some(ref stream) = self.stream {
            stream.play().expect("Failed to play the stream");
            self.is_playing = true;
            println!("[SENSIT_LOG] Playback started.");
        } else {
            println!("[ERROR] No stream available to play.");
        }
    }

    pub fn stop(&mut self) {
        // Simulate pausing by dropping the stream
        self.stream = None;
        self.is_playing = false;
        println!("[SENSIT_LOG] Playback stopped.");
    }

    pub fn next(&mut self) {
        self.audio_model.lock().unwrap().next_track();
        println!("[SENSIT_LOG] Playing next track...");
        self.load_current();
    }

    pub fn prev(&mut self) {
        self.audio_model.lock().unwrap().prev_track();
        println!("[SENSIT_LOG] Playing previous track...");
        self.load_current();
    }
}
