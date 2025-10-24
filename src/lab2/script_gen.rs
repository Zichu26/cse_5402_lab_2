/// File Name: script_gen.rs
/// Authors: Zichu Pan and Edgar Palomino
/// Summary: Contains the grab_trimmed_file_lines() function, which is used in the player.rs, play.rs and scene_fragment.rs modules
/// to get a Vector of Strings containing the trimmed lines of text from a target file

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use super::declarations::{FAILED_TO_OPEN_FILE, FAILED_TO_READ_LINE_FROM_FILE};

const EMPTY_LINE: usize = 0;

// The core function used for extracting data from files
// Used for both reading the config file line by line and reading the parts file line by line
pub fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    match File::open(&filename) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            loop {
                match reader.read_line(&mut line) {
                    Ok(EMPTY_LINE) => break,
                    Ok(_) => lines.push(line.trim().to_string()),
                    Err(error_code) => {
                        eprintln!("Error: Failed to read line from file '{}': {}", filename, error_code);
                        return Err(FAILED_TO_READ_LINE_FROM_FILE);
                    }
                }
                line.clear();
            }
            return Ok(());
        },
        Err(error_code) => {
            eprintln!("Error: Failed to open file '{}': {}", filename, error_code);
            return Err(FAILED_TO_OPEN_FILE);
        }
    }
}
