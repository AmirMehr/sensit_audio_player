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

bash

Copy code

`git clone <repository-url> cd <repository-directory>`

### Step 2: Build the Project

bash

Copy code

`cargo build`

### Step 3: Run the Application

bash

Copy code

`cargo run`

Project Structure
-----------------

bash

Copy code

`src/ ├── main.rs              # Application entry point ├── cli_view.rs          # Handles CLI interactions ├── player_controller.rs # Controls audio playback └── audio_model.rs       # Manages audio file handling`

