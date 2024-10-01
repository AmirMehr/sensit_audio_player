use std::fs;
use std::path::PathBuf;

pub struct AudioModel {
    pub files: Vec<PathBuf>,
    pub current_index: usize,
}

impl AudioModel {
    pub fn new(folder_path: &str) -> Self {
        let mut files = Vec::new();

        // Read the folder and filter audio files
        let paths = fs::read_dir(folder_path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path.extension().and_then(|s| s.to_str()).map(|s| s == "mp3" || s == "wav").unwrap_or(false) {
                files.push(path);
            }
        }

        Self {
            files,
            current_index: 0,
        }
    }

    // Get the current audio file
    pub fn get_current_file(&self) -> &PathBuf {
        &self.files[self.current_index]
    }

    // Move to the next track, wrapping around
    pub fn next_track(&mut self) {
        self.current_index = (self.current_index + 1) % self.files.len();
    }

    // Move to the previous track, wrapping around
    pub fn prev_track(&mut self) {
        if self.current_index == 0 {
            self.current_index = self.files.len() - 1;
        } else {
            self.current_index -= 1;
        }
    }
}
