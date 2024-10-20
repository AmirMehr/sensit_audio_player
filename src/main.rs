mod modules;

use modules::services::audio_folder_service::AudioFolderService;
use modules::services::audio_loader::{AudioLoader, DynamicAudioLoader};

use crate::modules::controllers::player_controller::PlayerController;
use crate::modules::models::audio_folder_model::AudioFolderModel;
use crate::modules::views::cli_view::{read_folder_input, start_cli};
use std::env;
use std::sync::{Arc, Mutex};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Step 1: Read the folder path from the user
    let folder = read_folder_input();

    // Step 2: Use AudioService to load the audio files from the folder
    let audio_files = AudioFolderService::load_audio_files(&folder);

    // Step 3: Create an instance of AudioModel with the loaded files
    let audio_model = Arc::new(Mutex::new(AudioFolderModel::new(audio_files)));

    // Step 4: Create an instance of DynamicAudioLoader wrapped in Box
    let audio_loader: Box<dyn AudioLoader> = Box::new(DynamicAudioLoader);

    // Step 5: Pass the audio loader and the audio model into the PlayerController
    let mut player_controller = PlayerController::new(audio_model.clone(), audio_loader);

    // Step 6: Load the current audio track (if available)
    player_controller.load_current();

    // Step 7: Start the CLI interface
    start_cli(&mut player_controller);
}
