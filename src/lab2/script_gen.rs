use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use super::declarations::{SCRIPT_GENERATION_ERROR};

pub fn grab_trimmed_file_lines(filename: &String, lines: &mut Vec<String>) -> Result<(), u8> {
    // The core function used for extracting data from files
    // Used for both reading the config file line by line and reading the parts file line by line
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(error_code) => {
            eprintln!("Error: Failed to open file '{}': {}", filename, error_code);
            return Err(SCRIPT_GENERATION_ERROR);
        }
    };
    
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    
    loop {
        line.clear();
        
        let bytes_read = match reader.read_line(&mut line) {
            Ok(bytes) => bytes,
            Err(error_code) => {
                eprintln!("Error: Failed to read line from file '{}': {}", filename, error_code);
                return Err(SCRIPT_GENERATION_ERROR);
            }
        };
        
        if bytes_read == 0 {
            return Ok(());
        }

        lines.push(line.trim().to_string());
    }
}


    