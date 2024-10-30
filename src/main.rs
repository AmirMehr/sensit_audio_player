mod modules;

use modules::services::audio_folder_service::AudioFolderService;
use modules::services::audio_loader::{AudioLoader, DynamicAudioLoader};

use modules::controllers::player_controller::PlayerController;
use modules::models::audio_folder_model::AudioFolderModel;
use modules::views::cli_view::CliView;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Create an instance of AudioFolderService.
    let audio_folder_service = AudioFolderService;

    // Use the service to read a valid folder input from the user.
    let audio_files = CliView::read_folder_input(&audio_folder_service);

    // Step 3: Create an instance of AudioModel with the loaded files
    let audio_model = AudioFolderModel::new(audio_files);

    // Step 4: Create an instance of DynamicAudioLoader wrapped in Box
    // I used a trait instead of a concrete type to make the code more expandable and to follow DI principles, reducing coupling between components and avoiding direct dependencies.
    let audio_loader: Box<dyn AudioLoader> = Box::new(DynamicAudioLoader);

    // Step 5: Pass the audio loader and the audio model into the PlayerController
    let mut player_controller = PlayerController::new(audio_model, audio_loader);

    // Step 6: Load the current audio track (if available)
    // player_controller.load_current();

    // Step 7: Start the CLI interface
    CliView::start_cli(&mut player_controller);
}
