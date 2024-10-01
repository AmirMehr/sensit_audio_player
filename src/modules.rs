// Central module declaration file for the project

// Declare the controllers module
pub mod controllers {
    pub mod player_controller;   // Expose player controller
}

// Declare the models module
pub mod models {
    pub mod audio_model;  // Expose the audio model
}

// Declare the views module
pub mod views {
    pub mod cli_view;  // Expose CLI view logic
}

