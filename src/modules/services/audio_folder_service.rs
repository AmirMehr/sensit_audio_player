use std::fs;
use std::path::PathBuf;

pub struct AudioFolderService;

/// **AudioFolderService Implementation**
///
/// This implementation provides functionality to load audio files from a specified folder path.
/// It reads the contents of a directory, filters audio files based on their extension (e.g., `.mp3` and `.wav`),
/// and returns the paths of the valid audio files.
///
/// # Usage
/// - Use this service to retrieve a list of audio files from a user-specified folder.
/// - It handles invalid folder paths gracefully by returning an empty vector.
///
/// # Example
/// ```
/// let audio_service = AudioFolderService;
/// let audio_files = audio_service.load_audio_files("/path/to/audio");
/// if audio_files.is_empty() {
///     println!("No audio files found.");
/// } else {
///     println!("Loaded {} audio files.", audio_files.len());
/// }
/// ```
impl AudioFolderService {
    /// **Load Audio Files from Folder**
    ///
    /// This function reads the specified folder path and returns the paths of audio files
    /// with supported extensions (currently `.mp3` and `.wav`).
    ///
    /// # Parameters
    /// - `folder_path`: A string slice representing the path to the folder containing audio files.
    ///
    /// # Returns
    /// - A `Vec<PathBuf>` containing the paths of all audio files found in the folder.
    /// - If the folder is invalid or contains no supported audio files, it returns an empty vector.
    ///
    /// # Behavior
    /// - If the folder cannot be read (e.g., it doesn’t exist), it returns an empty vector.
    /// - Filters files by their extension (`mp3`, `wav`). This can be extended to support more formats.
    ///
    /// # Example
    /// ```
    /// let audio_files = AudioFolderService.load_audio_files("/path/to/folder");
    /// if audio_files.is_empty() {
    ///     println!("No audio files found or invalid folder path.");
    /// }
    /// ```
    pub fn load_audio_files(&self, folder_path: &str) -> Vec<PathBuf> {
        let mut files = Vec::new();

        // Attempt to read the directory. If it fails, return an empty vector.
        let paths = match fs::read_dir(folder_path) {
            Ok(paths) => paths,
            Err(_) => return files, // Invalid folder path.
        };

        // Iterate through the directory and collect valid audio files.
        for path in paths {
            if let Ok(entry) = path {
                let path = entry.path();
                // Check if the file has a supported audio extension.
                if path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s == "mp3" || s == "wav")
                    .unwrap_or(false)
                {
                    files.push(path); // Add valid audio file path to the vector.
                }
            }
        }

        files // Return the vector of valid audio file paths.
    }
}
