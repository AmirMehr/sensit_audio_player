use std::io::{self, Write};
use crate::modules::controllers::player_controller::PlayerController;

pub fn start_cli(player_controller: &mut PlayerController) {
    loop {
        // Get user input
        print!("Enter command (p = play/pause, j = prev, k = next, q = quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "p" => player_controller.toggle_play(),
            "j" => player_controller.prev(),
            "k" => player_controller.next(),
            "q" => break,
            _ => println!("Invalid command"),
        }
    }
}

pub fn read_folder_input() -> String {
    print!("Enter the folder path containing audio files: ");
    io::stdout().flush().unwrap();

    let mut folder = String::new();
    io::stdin().read_line(&mut folder).unwrap();
    folder.trim().to_string()
}
