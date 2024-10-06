* * *

CLI Audio Player
================

This is a command-line audio player built with Rust. It supports playing **WAV** files and basic controls such as play, pause, next track, and previous track.

Features
--------

*   Play/Pause audio tracks
*   Navigate through tracks (Next/Previous)
*   Supports `.wav` audio files
*   Command-line interface (CLI) for control

Requirements
------------

*   Rust (latest version)
*   Cargo (included with Rust)

Build and Run
-------------

### Step 1: Clone the Repository
```
git clone [<repository-url>](https://github.com/AmirMehr/sensit_audio_player.git)
```

```
cd sensit_audio_player
```

### Step 2: Build the Project
```
cargo build
```

### Step 3: Run the Application
```
cargo run
```

Project Structure
-----------------

```
root/
├── Cargo.toml
└── src/
    ├── main.rs              # Application entry point
    ├── module.rs            # Module registry
    └── modules/
        ├── controllers/
        │   └── player_controller.rs    # Controls audio playback
        ├── models/
        │   └── audio_folder_model.rs   # Manages audio data and tracks
        ├── services/
        │   ├── audio_folder_service.rs # Service to read audio folder content
        │   ├── audio_loader.rs         # Service to load audio data
        │   ├── wav_loader.rs           # WAV-specific loader
        │   └── mp3_loader.rs           # MP3-specific loader
        └── views/
            └── cli_view.rs             # Handles command-line interface
```