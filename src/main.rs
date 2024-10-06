mod modules;

use modules::services::wav_loader::WavLoader;

use crate::modules::controllers::player_controller::PlayerController;
use crate::modules::models::audio_model::AudioModel;
use crate::modules::views::cli_view::{read_folder_input, start_cli};
use std::sync::{Arc, Mutex};

fn main() {
    // Read folder path from the user
    let folder = read_folder_input();


    // Step 1: Create an instance of your audio model
    let audio_model = Arc::new(Mutex::new(AudioModel::new(&folder))); // Adjust based on how your AudioModel is set up

    // Step 2: Create an instance of WavLoader
    let wav_loader = WavLoader;

    // Step 3: Pass WavLoader and the audio model into the PlayerController
    let mut player_controller = PlayerController::new(audio_model.clone(), wav_loader);

    player_controller.load_current();
    
    // Start the CLI interface
    start_cli(&mut player_controller);
}
