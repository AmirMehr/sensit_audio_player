mod modules;
use crate::modules::controllers::player_controller::PlayerController;
use crate::modules::models::audio_model::AudioModel;
use crate::modules::views::cli_view::{read_folder_input, start_cli};
use std::sync::{Arc, Mutex};

fn main() {
    // Read folder path from the user
    let folder = read_folder_input();

    // Initialize the audio model with the files from the folder
    // @TODO in a production mode, we should implement logic for re-asking a path if path is wrong or doesn't have audio files
    let model: Arc<Mutex<AudioModel>> = Arc::new(Mutex::new(AudioModel::new(&folder)));

    // Initialize the player controller
    let mut player_controller: PlayerController = PlayerController::new(Arc::clone(&model));

    // Start the CLI interface
    start_cli(&mut player_controller);
}
