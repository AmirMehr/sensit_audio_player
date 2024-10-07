use std::path::PathBuf;

pub struct AudioFolderModel {
    pub files: Vec<PathBuf>,
    pub current_index: usize,
}

impl AudioFolderModel {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self {
            files,
            current_index: 0,
        }
    }

    // Get the current audio file
    pub fn get_current_file(&self) -> &PathBuf {
        &self.files[self.current_index]
    }

    // Move to the next track
    pub fn next_track(&mut self) {
        self.current_index = (self.current_index + 1) % self.files.len();
    }

    // Move to the previous track
    pub fn prev_track(&mut self) {
        if self.current_index == 0 {
            self.current_index = self.files.len() - 1;
        } else {
            self.current_index -= 1;
        }
    }
}
