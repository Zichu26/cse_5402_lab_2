/// File Name: player.rs
/// Author: Zichu Pan and Edgar Palomino
/// Summary: This module implements the Player structure that represents individual actors/characters in a play, 
/// managing their dialogue lines and delivery.

use std::sync::atomic::Ordering as AtomicOrdering;
use std::cmp::Ordering;

use super::declarations::WHINGE_MODE;
use super::script_gen::grab_trimmed_file_lines;

pub type PlayLines = Vec<(usize, String)>; // (line_number, line_text)

const FIRST_CHARACTER_LINE: usize = 0;
const CHARACTER_LINE_STEP: usize = 1;

pub struct Player {
    name: String,
    lines: PlayLines,
    index: usize
}

impl Player {

    // Create a new player
    pub fn new(name: &String) -> Player {
        Player {name: name.clone(), lines: PlayLines::new(), index: FIRST_CHARACTER_LINE}
    }

    /// Parses individual script lines:
    /// - Expects format: <line_number> <dialogue_text>
    /// - Extracts line number from first token
    /// - Stores the remaining text as dialogue
    /// - Warns about invalid line numbers in whinge mode
    /// - Ignores empty lines
    fn add_script_line(&mut self, line: &String) {
        // Ignore empty lines
        if !line.is_empty() {
            if let Some((first_token, rest_of_line)) = line.split_once(char::is_whitespace) {
                // Try to parse the first token as line number
                if let Ok(line_number) = first_token.parse::<usize>() {
                    // Remove leading and trailing whitespace before inserting the character line
                    self.lines.push((line_number, rest_of_line.trim().to_string()));
                } else if WHINGE_MODE.load(AtomicOrdering::SeqCst) {
                    println!("Warning: '{}' does not represent a valid line number", first_token);
                }
            }
        }
    }

    /// Loads the player's script:
    /// - Reads lines from the character's script file
    /// - Parses each line using add_script_line()
    /// - Sorts lines by line number to handle out-of-order input
    pub fn prepare(&mut self, part_filename: &String) -> Result<(), u8> {

        let mut part_lines: Vec<String> = Vec::new();
        
        if let Err(error_code) = grab_trimmed_file_lines(part_filename, &mut part_lines) {
            return Err(error_code);
        }

        // Process each line and add to player's lines
        for line in &part_lines {
            self.add_script_line(line);
        }

        // Sort lines by line number to handle out-of-order lines
        self.lines.sort();

        Ok(())

    }

    /// Delivers the next line of dialogue:
    /// - Checks if all lines have been spoken
    /// - Prints character name if speaker changes
    /// - Prints the dialogue text
    /// - Advances the index to next line
    pub fn speak(&mut self, current_speaker: &mut String) {

        // Return if all lines have already been spoken
        if !(self.index < self.lines.len()) {
            return;
        }

        // Check if this player is different from the current speaker
        if self.name != *current_speaker {
            // Update the current speaker to this player's name
            *current_speaker = self.name.clone();
            println!();
            println!("{}.", current_speaker);
        }

        let (_line_number, line_text) = &self.lines[self.index];

        println!("{}", line_text);

        self.index += CHARACTER_LINE_STEP;

    }

    /// Returns the number of the next line if the character still has lines to read
    /// and None if the character has read all their lines
    pub fn next_line(&self) -> Option<usize> {

        if !(self.index < self.lines.len()) {
            return None;
        }

        let (line_number, _line_text) = &self.lines[self.index];

        Some(*line_number)

    }

    /// Getter method for the character_name to facilitate printing in scene_fragment.rs
    pub fn get_character_name(&self) -> &String {
        &self.name
    }

}

// Implementing PartialEq, Eq, PartialOrd and Ord traits to allow for sorting of players in scene_fragment.rs

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        match (self.lines.is_empty(), other.lines.is_empty()) {
            // If both players have no lines to speak, they are equal
            (true, true) => true,
            // If both players have lines to speak, they are equal if they have the first line number
            (false, false) => {
                let (line_number, _line_text) = &self.lines[FIRST_CHARACTER_LINE];
                let (other_line_number, _other_line_text) = &other.lines[FIRST_CHARACTER_LINE];
                line_number == other_line_number
            },
            // If only one player has lines to speak, they are different
            _ => false
        }
    }
}

impl Eq for Player { }

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.lines.is_empty(), other.lines.is_empty()) {
            // If both players have no lines to speak, they are equal
            (true, true) => Ordering::Equal,
            // If this player has lines to speak but the other doesn't, this player goes before
            (true, false) => Ordering::Less,
            // If the other player has lines to speak but this one doesn't, the other player goes before
            (false, true) => Ordering::Greater,
            // If both players have lines to speak, the one with the earliest first line number goes before
            (false, false) => {
                let (line_number, _line_text) = &self.lines[FIRST_CHARACTER_LINE];
                let (other_line_number, _other_line_text) = &other.lines[FIRST_CHARACTER_LINE];
                line_number.cmp(other_line_number)
            }
        }
    }
}
