use std::fs;
use std::path::PathBuf;

pub struct AudioFolderService;

impl AudioFolderService {
    pub fn load_audio_files(folder_path: &str) -> Vec<PathBuf> {
        let mut files = Vec::new();

        // Read the folder and filter audio files
        let paths = fs::read_dir(folder_path).unwrap();
        for path in paths {
            let path = path.unwrap().path();
            if path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s == "mp3" || s == "wav")
                .unwrap_or(false)
            {
                files.push(path);
            }
        }

        files
    }
}
