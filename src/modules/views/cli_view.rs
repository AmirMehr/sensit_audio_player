use std::io::{self, Write};

use crate::modules::{
    controllers::player_controller::PlayerController,
    services::audio_folder_service::AudioFolderService,
};

pub struct CliView;

impl CliView {
    /// Starts the command-line interface (CLI) to control audio playback.
    ///
    /// # Parameters
    /// - `player_controller`: A mutable reference to the `PlayerController`
    ///   used to control playback operations (play/pause, previous, next).
    ///
    /// # Behavior
    /// - Continuously prompts the user to enter a command.
    /// - Commands:
    ///     - `p`: Toggle play/pause.
    ///     - `j`: Play the previous track.
    ///     - `k`: Play the next track.
    ///     - `q`: Quit the CLI.
    /// - If an invalid command is entered, it displays a message and waits for new input.
    ///
    /// # Panics
    /// - If `stdout` flush or `stdin` read fails, it will panic with `unwrap()`.
    pub fn start_cli(player_controller: &mut PlayerController) {
        loop {
            // Get user input
            print!(
                "🎵 Enter command (▶️  p = play/pause, ⏮️  j = prev, ⏭️  k = next, 🛑 q = quit): "
            );
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            match input {
                "p" => player_controller.toggle_play(),
                "j" => player_controller.prev(),
                "k" => player_controller.next(),
                "q" => break, // break is enought nothing will happen after here in the programm
                _ => println!("Invalid command"),
            }
        }
    }

    /// Prompts the user to enter a valid folder path containing audio files.
    ///
    /// # Parameters
    /// - `audio_folder_service`: A reference to the `AudioFolderService` to load audio files.
    ///
    /// # Returns
    /// A `Vec<PathBuf>` containing the paths of valid audio files.
    pub fn read_folder_input(audio_folder_service: &AudioFolderService) -> Vec<std::path::PathBuf> {
        loop {
            print!("📁 Enter the folder path containing audio files(q to quit): ");
            io::stdout().flush().unwrap(); // Ensure prompt is displayed immediately.

            let mut folder = String::new();
            io::stdin().read_line(&mut folder).unwrap();
            let folder = folder.trim().to_string();

            // Exit if the user enters "q"
            if folder == "q" {
                println!("Exiting CLI.");
                std::process::exit(0); // Exits the program
            }

            // Use the service to load audio files from the provided path.
            let audio_files = audio_folder_service.load_audio_files(&folder);

            if audio_files.is_empty() {
                println!("⚠️  No audio files found or invalid path. Please try again.");
            } else {
                return audio_files; // Return valid audio files.
            }
        }
    }
}
