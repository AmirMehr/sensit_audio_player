// Central module declaration file for the project

// Declare the controllers module
pub mod controllers {
    pub mod player_controller; // Expose player controller
}

// Declare the services module
pub mod services {
    pub mod audio_folder_service;
    pub mod audio_loader; // Expose audio loader
    pub mod mp3_loader; // Expose mp3 loader
    pub mod wav_loader; // Expose wav loader
}

// Declare the models module
pub mod models {
    pub mod audio_folder_model; // Expose the audio model, It can be renamed to album
}

// Declare the views module
pub mod views {
    pub mod cli_view; // Expose CLI view logic
}
