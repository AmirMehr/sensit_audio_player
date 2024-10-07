mod modules;

use modules::services::audio_folder_service::AudioFolderService;
use modules::services::wav_loader::WavLoader;

use crate::modules::controllers::player_controller::PlayerController;
use crate::modules::models::audio_folder_model::AudioFolderModel;
use crate::modules::views::cli_view::{read_folder_input, start_cli};
use std::sync::{Arc, Mutex};

fn main() {
    // Step 1: Read the folder path from the user
    let folder = read_folder_input();

    // Step 2: Use AudioService to load the audio files from the folder
    let audio_files = AudioFolderService::load_audio_files(&folder);

    // Step 3: Create an instance of AudioModel with the loaded files
    let audio_model = Arc::new(Mutex::new(AudioFolderModel::new(audio_files)));

    // Step 4: Create an instance of WavLoader
    // @TODO: In a production mode, we are able to implement a logic for selecting loader based on the selected audio
    // For example: Read the type of file, and if it is a .mp3 file then with a Factory pattern we are able to have a Mp3Loader and pass it to the controller or create in inside
    let wav_loader = WavLoader;

    // Step 5: Pass WavLoader(Or Mp3Loader) and the audio model into the PlayerController
    let mut player_controller = PlayerController::new(audio_model.clone(), wav_loader);

    // Step 6: Load the current audio track (if available)
    player_controller.load_current();

    // Step 7: Start the CLI interface
    start_cli(&mut player_controller);
}
