/// File Name: scene_fragment.rs
/// Authors: Zichu Pan and Edgar Palomino
/// Summary: This module implements the SceneFragment structure that represents individual scenes within a play, 
/// managing players (actors) and their dialogue.

use std::sync::atomic::Ordering as AtomicOrdering;

use super::player::Player;
use super::declarations::{WHINGE_MODE, CONFIG_PARSING_ERROR};
use super::script_gen::grab_trimmed_file_lines;

pub type PlayConfig = Vec<(String, String)>; // (part_name, part_filename)

pub const PART_NAME_INDEX: usize = 0;
pub const PART_FILENAME_INDEX: usize = 1;
pub const CONFIG_LINE_TOKEN_COUNT: usize = 2;
pub const CHARACTER_LINE_STEP: usize = 1;

pub struct SceneFragment {
    title: String,
    players: Vec<Player>
}

impl SceneFragment {

    // Create a new SceneFragment
    pub fn new(title: &String) -> SceneFragment {
        SceneFragment {title: title.clone(), players: Vec::new()}
    }

    /// Instantiates Player objects:
    /// - Creates a Player for each character
    /// - Calls prepare() on each player with their script file
    pub fn process_config(&mut self, config: &PlayConfig) -> Result<(), u8> {

        for config_entry in config {
            match config_entry {
                (part_name, part_filename) => {
                    // Create a new Player instance using the part name
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

    /// Adds a configuration line to the config vector, ensuring that it has at least
    /// the minimum number of tokens for a configuration line (2) and complaining
    /// if this is not the case
    fn add_config(line: &String, config: &mut PlayConfig) {

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() < CONFIG_LINE_TOKEN_COUNT && WHINGE_MODE.load(AtomicOrdering::SeqCst) {
            eprintln!("Warning: Configuration line has too few tokens (expected {}, got {}): '{}'", CONFIG_LINE_TOKEN_COUNT, tokens.len(), line);
        } else if tokens.len() > CONFIG_LINE_TOKEN_COUNT && WHINGE_MODE.load(AtomicOrdering::SeqCst) {
            eprintln!("Warning: Configuration line has too many tokens (expected {}, got {}): '{}'", CONFIG_LINE_TOKEN_COUNT, tokens.len(), line);
        }

        if tokens.len() >= CONFIG_LINE_TOKEN_COUNT {
            config.push((tokens[PART_NAME_INDEX].to_string(), tokens[PART_FILENAME_INDEX].to_string()));
        }

    }

    /// Parse configuration files:
    /// - Each line should have exactly 2 tokens: character name and their script file
    /// - Warns about malformed lines (too few/many tokens) in whinge mode
    /// - Builds a PlayConfig with character-to-script mappings
    pub fn read_config(config_filename: &String, config: &mut PlayConfig) -> Result<(), u8> {

        let mut config_lines: Vec<String> = Vec::new();

        if let Err(error_code) = grab_trimmed_file_lines(config_filename, &mut config_lines) {
            return Err(error_code);
        }

        if config_lines.is_empty() {
            eprintln!("Error: Config file '{}' contains no lines", config_filename);
            return Err(CONFIG_PARSING_ERROR);
        }

        for line in &config_lines {
            SceneFragment::add_config(line, config);
        }

        Ok(())

    }

    /// Main setup method that:
    /// - Reads the configuration file for this scene
    /// - Creates and prepares Player objects for each character
    /// - Sorts players by line number
    pub fn prepare(&mut self, config_filename: &String) -> Result<(), u8> {
        
        let mut config: PlayConfig = Vec::new();
        
        if let Err(error_code) = SceneFragment::read_config(config_filename, &mut config) {
            return Err(error_code);
        }

        if let Err(error_code) = self.process_config(&config) {
            return Err(error_code);
        }

        self.players.sort();

        Ok(())

    }

    /// Helper function to check if the title field of the SceneFragment struct is empty from play.rs
    pub fn has_scene_title(&self) -> bool {
        !self.title.trim().is_empty()
    }

    /// Helper function to print the title of the scene
    fn print_scene_title(&self, is_first_scene: bool) {
        if self.has_scene_title() {
            if !is_first_scene {
                 // Adding a blank line before the scene title unless it's the first scene
                println!();
            }
            println!("{}", self.title);
            println!();
        }
    }

    // Implementing functions to handle the players entrances and exits
    // (both group and individual at the start and end of each scene)

    pub fn enter(&self, previous: &SceneFragment) {
        self.print_scene_title(false);
        for player in &self.players {
            // Check if player was in previous scene
            let player_was_in_previous_scene = previous.players.iter().any(|p| p.get_character_name() == player.get_character_name());
            if !player_was_in_previous_scene {
                println!("[Enter {}.]", player.get_character_name());
            }
        }
    }

    pub fn enter_all(&self) {    
        self.print_scene_title(true);
        for player in &self.players {
            println!("[Enter {}.]", player.get_character_name());
        }
    }

    pub fn exit(&self, next: &SceneFragment) {
        for player in self.players.iter().rev() {
            // Check if this player will be in next scene
            let player_will_be_in_next_scene = next.players.iter().any(|p| p.get_character_name() == player.get_character_name());
            if !player_will_be_in_next_scene {
                println!("[Exit {}.]", player.get_character_name());
            }
        }
    }

    pub fn exit_all(&self) {
        for player in self.players.iter().rev() {
            println!("[Exit {}.]", player.get_character_name());
        }
    }

    /// Orchestrates dialogue delivery:
    /// - Repeatedly finds the player with the smallest next line number
    /// - That player speaks their line
    /// - Tracks expected line numbers to detect missing/duplicate lines
    /// -  Warns about line number issues in whinge mode
    /// - Continues until all players have delivered all lines
    pub fn recite(&mut self) {

        let mut current_speaker = String::new();
        let mut expected_line_number: usize = 0;
        
        loop {

            // Find the player with the smallest next line number
            let mut next_line_number: Option<usize> = None;
            let mut next_player_index: Option<usize> = None;

            for (index, player) in self.players.iter().enumerate() {
                if let Some(line_num) = player.next_line() {
                    // If next_line_number is None, it means that a player with a line hasn't been found yet in this iteration
                    // and if line_num is less than the unwrapped value of next_line_number, it means that we've found the first
                    // player who has lines remaining, where the next one would be the next line number by default
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
                if WHINGE_MODE.load(AtomicOrdering::SeqCst) {
                    for missing in expected_line_number..actual_line_number {
                        eprintln!("Warning: Missing line number {}", missing);
                    }
                }
                expected_line_number = actual_line_number;
            }

            if actual_line_number == expected_line_number {
                // Check for matching line numbers and since this is the expected line, advance the counter
                expected_line_number += CHARACTER_LINE_STEP;
            } else if actual_line_number < expected_line_number && WHINGE_MODE.load(AtomicOrdering::SeqCst) {
                // This is a duplicate line number
                eprintln!("Warning: Duplicate line number {}", actual_line_number);
            }

            // Have the selected player speak their line
            let player_index = next_player_index.unwrap();
            self.players[player_index].speak(&mut current_speaker);

        }

    }

}
