extern crate cpal;

use crate::modules::models::audio_folder_model::AudioFolderModel;
use crate::modules::services::audio_loader::AudioLoader;
use cpal::traits::StreamTrait;
use cpal::Stream;
use std::path::PathBuf;
use std::time::Instant;

/// **PlayerController Struct**
///
/// This struct manages the playback of audio files using the `AudioFolderModel`
/// and `AudioLoader`. It holds the current stream state, manages play/pause operations,
/// and allows navigation between tracks.
pub struct PlayerController {
    audio_model: AudioFolderModel, // Manages the collection and current index of audio files.
    audio_loader: Box<dyn AudioLoader>, // Uses dynamic dispatch to load audio streams based on the file format.
    stream: Option<Stream>,             // Holds the currently playing audio stream, if any.
    is_playing: bool,                   // Tracks whether audio playback is currently active.
}

impl PlayerController {
    /// **Constructor for PlayerController**
    ///
    /// Creates a new instance of `PlayerController` by accepting an audio model
    /// and an audio loader that implements the `AudioLoader` trait.
    ///
    /// # Parameters:
    /// - `audio_model`: The model holding the list of audio files and the current track index.
    /// - `audio_loader`: A boxed trait object that loads audio streams dynamically.
    ///
    /// # Returns:
    /// - A new instance of `PlayerController`.
    pub fn new(audio_model: AudioFolderModel, audio_loader: Box<dyn AudioLoader>) -> Self {
        let mut instance = PlayerController {
            audio_model,
            audio_loader,
            stream: None,
            is_playing: false,
        };

        instance.load_current();
        // Maybe call play here if we want to play immediately after first load
        // instance.play() ;
        instance
    }

    /// **Load the Current Audio Track**
    ///
    /// Loads the current audio file from the `AudioFolderModel` and creates an audio stream.
    ///
    /// # Returns:
    /// - `Ok(())` if the stream is successfully loaded.
    /// - `Err(Box<dyn Error>)` if loading fails.
    pub fn load_current(&mut self) {
        let current_file = self.get_current_file(); // Get the current audio file path.
        println!("[SENSIT_LOG] Loading file: {:?}", current_file.display());

        let start_time = Instant::now(); // Start measuring the time taken to load the stream.

        // Use the audio loader to create an audio stream for the current file.
        match self.audio_loader.create_audio_stream(&current_file) {
            Ok(stream) => {
                self.stream = Some(stream); // Store the created stream.
                println!(
                    "[SENSIT_LOG] Stream created successfully for file {:?} in {:?}.",
                    current_file.display(),
                    start_time.elapsed() // Log the time taken to create the stream.
                );
            }
            Err(err) => {
                eprintln!("[ERROR] Failed to create stream: {:?}", err); // Log the error.
                self.stream = None; // Reset the stream if loading fails.
            }
        }
    }

    /// **Get the Current Audio File**
    ///
    /// Returns the path of the current audio file from the `AudioFolderModel`.
    ///
    /// # Returns:
    /// - `PathBuf`: The path to the current audio file.
    fn get_current_file(&self) -> PathBuf {
        self.audio_model.get_current_file().to_path_buf()
    }

    /// **Toggle Playback State**
    ///
    /// Toggles between playing and pausing the audio stream.
    pub fn toggle_play(&mut self) {
        if self.is_playing {
            self.pause(); // If playing, pause the stream.
        } else {
            self.play(); // If paused, start playback.
        }
    }

    /// **Play the Current Audio Stream**
    ///
    /// Starts playback of the current audio stream. If no stream is available,
    /// it attempts to load the stream and plays it if loading is successful.
    fn play(&mut self) {
        if let Some(ref stream) = self.stream {
            // If a stream is already available, start playback.
            stream.play().expect("Failed to play the stream");
            self.is_playing = true;
            println!("[SENSIT_LOG] Playback started.");
        } else {
            // No stream available, attempt to load the current track.
            println!("[SENSIT_LOG] No stream available.");
        }
    }

    /// **Pause the Current Audio Stream**
    ///
    /// Pauses the playback of the current audio stream. If no stream is available,
    /// it logs an error.
    fn pause(&mut self) {
        if let Some(ref stream) = self.stream {
            // If a stream is available, pause it.
            stream.pause().expect("Failed to pause the stream");
            self.is_playing = false; // Update the playback state.
            println!("[SENSIT_LOG] Playback paused.");
        } else {
            // Log an error if no stream is available.
            println!("[ERROR] No stream available to pause.");
        }
    }

    /// **Play the Next Track**
    ///
    /// Advances to the next track in the `AudioFolderModel` and starts playback.
    pub fn next(&mut self) {
        println!("[SENSIT_LOG] Playing next track...");
        self.audio_model.next_track(); // Move to the next track.
        self.load_current(); // Load and play the next track.
        self.play();
    }

    /// **Play the Previous Track**
    ///
    /// Moves to the previous track in the `AudioFolderModel` and starts playback.
    pub fn prev(&mut self) {
        println!("[SENSIT_LOG] Playing previous track...");
        self.audio_model.prev_track(); // Move to the previous track.
        self.load_current(); // Load and play the previous track.
        self.play();
    }
}
