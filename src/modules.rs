// Central module declaration file for the project

// Declare the controllers module
pub mod controllers {
    pub mod player_controller; // Expose player controller
}

// Declare the services module
pub mod services {
    pub mod audio_loader; // Expose audio loader
    pub mod mp3_loader;
    pub mod wav_loader; // Expose wav loader // Expose mp3 loader
}

// Declare the models module
pub mod models {
    pub mod audio_model; // Expose the audio model
}

// Declare the views module
pub mod views {
    pub mod cli_view; // Expose CLI view logic
}
