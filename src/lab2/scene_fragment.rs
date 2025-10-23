use std::sync::atomic::Ordering;
use super::player::Player;
use super::declarations::{WHINGE_MODE, SCRIPT_GENERATION_ERROR};
use super::script_gen::grab_trimmed_file_lines;

pub type PlayConfig = Vec<(String, String)>; // (character_name, part_file_name)

pub const TITLE_LINE_INDEX: usize = 0;        
pub const FIRST_CHARACTER_LINE_INDEX: usize = 1; 
pub const CHARACTER_NAME_INDEX: usize = 0;
pub const PART_FILE_NAME_INDEX: usize = 1;
pub const CONFIG_LINE_TOKEN_COUNT: usize = 2;

pub struct SceneFragment {
    title: String,
    players: Vec<Player>,
}

impl SceneFragment {
    pub fn new(title: &String) -> SceneFragment {
        SceneFragment {
            title: title.clone(),
            players: Vec::new(),
        }
    }

    pub fn process_config(&mut self, config: &PlayConfig) -> Result<(), u8> {
        for config_entry in config {
            match config_entry {
                (part_name, part_filename) => {
                    // Create a new Player instance using the character name
                    let mut player = Player::new(part_name);
                    
                    // Call prepare on the player with the part filename
                    if let Err(error_code) = player.prepare(part_filename) {
                        return Err(error_code);
                    }
                    
                    // Push the prepared player into the Play's vector
                    self.players.push(player);
                }
            }
        }
        Ok(())
    }

    fn add_config(&mut self, config_line: &String, config: &mut PlayConfig) {
        let tokens: Vec<&str> = config_line.split_whitespace().collect();
        
        if tokens.len() != CONFIG_LINE_TOKEN_COUNT {
            if WHINGE_MODE.load(Ordering::SeqCst) {
                if tokens.len() < CONFIG_LINE_TOKEN_COUNT {
                    eprintln!("Warning: Configuration line has too few tokens (expected {}, got {}): '{}'", 
                            CONFIG_LINE_TOKEN_COUNT, tokens.len(), config_line);
                } else {
                    eprintln!("Warning: Configuration line has too many tokens (expected {}, got {}): '{}'", 
                            CONFIG_LINE_TOKEN_COUNT, tokens.len(), config_line);
                }
            }
        }
        
        if tokens.len() >= CONFIG_LINE_TOKEN_COUNT {
            config.push((
                tokens[CHARACTER_NAME_INDEX].to_string(),
                tokens[PART_FILE_NAME_INDEX].to_string()
            ));
        }
    }

    pub fn read_config(&mut self, config_filename: &String, config: &mut PlayConfig) -> Result<(), u8> {
        let mut config_lines: Vec<String> = Vec::new();
        
        if let Err(_error_code) = grab_trimmed_file_lines(config_filename, &mut config_lines) {
            return Err(SCRIPT_GENERATION_ERROR);
        }
        
        if config_lines.len() < 2 {
            eprintln!("Error: Configuration file must contain at least 2 lines (title and one character)");
            return Err(SCRIPT_GENERATION_ERROR);
        }
        
        for line in &config_lines {
            self.add_config(line, config);
        }
        
        Ok(())
    }

    pub fn has_title(&self) -> bool {
        !self.title.trim().is_empty()
    }

    pub fn prepare(&mut self, config_filename: &String) -> Result<(), u8> {
        let mut config: PlayConfig = Vec::new();
        
        if let Err(_error_code) = self.read_config(config_filename, &mut config) {
            return Err(SCRIPT_GENERATION_ERROR);
        }
        
        if let Err(_error_code) = self.process_config(&config) {
            return Err(SCRIPT_GENERATION_ERROR);
        }

        self.players.sort();
        
        Ok(())
    }

    fn print_title(&self) {
        if !self.title.trim().is_empty() {
            println!("{}", self.title);
            println!();
        }
    }

    pub fn enter(&self, previous: &SceneFragment) {
        self.print_title();
        for player in &self.players {
            // Check if player was in the previous scene
            let in_previous = previous.players.iter().any(|p| p.name() == player.name());
            if !in_previous {
                println!("[Enter {}.]", player.name());
            }
        }
    }

    pub fn enter_all(&self) {    
        self.print_title();
        for player in &self.players {
            println!("[Enter {}.]", player.name());
        }
    }

    pub fn exit(&self, next: &SceneFragment) {
        for player in self.players.iter().rev() {
            // Check if this player will be in the next scene
            let in_next = next.players.iter().any(|p| p.name() == player.name());
            if !in_next {
                println!("[Exit {}.]", player.name());
            }
        }
    }

    pub fn exit_all(&self) {
        for player in self.players.iter().rev() {
            println!("[Exit {}.]", player.name());
        }
    }

    pub fn recite(&mut self) {
        let mut current_speaker = String::new();
        let mut expected_line_number: usize = 0;
        
        loop {
            // Find the player with the smallest next line number
            let mut next_line_number: Option<usize> = None;
            let mut next_player_index: Option<usize> = None;
            for (index, player) in self.players.iter().enumerate() {
                if let Some(line_num) = player.next_line() {
                    // next_line_number is_none() means we haven't found any player with a line yet in this iteration
                    // this is the first player we've encountered who has lines remaining, 
                    // so we should record their information.
                    if next_line_number.is_none() || line_num < next_line_number.unwrap() {
                        next_line_number = Some(line_num);
                        next_player_index = Some(index);
                    }
                }
            }
            // If no player has lines left, we're done
            if next_player_index.is_none() {
                break;
            }
            
            // Check for missing line numbers
            let actual_line_number = next_line_number.unwrap();
            if actual_line_number > expected_line_number {
                if WHINGE_MODE.load(Ordering::SeqCst) {
                    for missing in expected_line_number..actual_line_number {
                        eprintln!("Warning: Missing line number {}", missing);
                    }
                }
                expected_line_number = actual_line_number;
            }
            
            // Check for duplicate line numbers
            if actual_line_number == expected_line_number {
                // This is the expected line, advance the counter
                expected_line_number += 1;
            } else if actual_line_number < expected_line_number {
                // This is a duplicate
                if WHINGE_MODE.load(Ordering::SeqCst) {
                    eprintln!("Warning: Duplicate line number {}", actual_line_number);
                }
            }
            
            // Have the selected player speak their line
            let player_index = next_player_index.unwrap();
            self.players[player_index].speak(&mut current_speaker);
        }
    }

}