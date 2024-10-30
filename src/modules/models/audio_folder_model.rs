use std::path::PathBuf;

/// `AudioFolderModel` represents a collection of audio files with the capability to
/// track the current playing file's index and navigate between tracks.
pub struct AudioFolderModel {
    /// A vector containing the paths to all audio files.
    pub files: Vec<PathBuf>,
    /// The index of the currently playing track.
    pub current_index: usize,
}

impl AudioFolderModel {
    /// Creates a new `AudioFolderModel` instance with a list of audio files.
    ///
    /// # Arguments
    /// * `files` - A vector of `PathBuf` containing the paths to the audio files.
    ///
    /// # Returns
    /// * `AudioFolderModel` - A new instance initialized with the provided files.
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self {
            files,
            current_index: 0, // Start with the first track by default.
        }
    }

    /// Returns a reference to the `PathBuf` of the currently playing audio file.
    ///
    /// # Panics
    /// If the `files` vector is empty, this will panic due to an out-of-bounds access.
    ///
    /// # Example
    /// ```
    /// let current_file = audio_model.get_current_file();
    /// println!("Now playing: {:?}", current_file);
    /// ```
    pub fn get_current_file(&self) -> &PathBuf {
        &self.files[self.current_index]
    }

    /// Advances to the next track in the list.
    /// If the end of the list is reached, it wraps around to the first track.
    ///
    /// # Example
    /// ```
    /// audio_model.next_track();
    /// ```
    pub fn next_track(&mut self) {
        // Use modulo to wrap around to the first track if at the end of the list.
        self.current_index = (self.current_index + 1) % self.files.len();
    }

    /// Moves to the previous track in the list.
    /// If the current track is the first one, it wraps around to the last track.
    ///
    /// # Example
    /// ```
    /// audio_model.prev_track();
    /// ```
    pub fn prev_track(&mut self) {
        if self.current_index == 0 {
            // Wrap around to the last track if at the first track.
            self.current_index = self.files.len() - 1;
        } else {
            // Otherwise, move to the previous track.
            self.current_index -= 1;
        }
    }
}
