/// File Name: play.rs
/// Authors: Zichu Pan and Edgar Palomino
/// Summary: This module implements the core Play structure that orchestrates a performance by managing scene fragments.

use std::sync::atomic::Ordering as AtomicOrdering;

use super::scene_fragment::SceneFragment;
use super::declarations::{WHINGE_MODE, SCRIPT_PARSING_ERROR};
use super::script_gen::grab_trimmed_file_lines;

pub type ScriptConfig = Vec<(bool, String)>;
pub type Fragments = Vec<SceneFragment>;

const SCENE_SCRIPT_INDEX: usize = 0;
const SCENE_SCRIPT_LENGTH: usize = 2;
const SCENE_TITLE_START_INDEX: usize = 1;

const CONFIG_FILENAME_INDEX: usize = 0;
const CONFIG_SCRIPT_LENGTH: usize = 1;
const CONFIG_EXTRA_TOKENS_START_INDEX: usize = 1;

const FIRST_SCENE_FRAGMENT: usize = 0;
const SCENE_FRAGMENT_STEP: usize = 1;

pub struct Play {
    fragments: Fragments
}

impl Play {

    /// Create a new Play
    pub fn new() -> Play {
        Play {fragments: Vec::new()}
    }

    /// Converts the ScriptConfig into SceneFragment objects:
    /// - Scene titles are stored temporarily
    /// - Config filenames trigger creation of new fragments with the current title
    pub fn process_config(&mut self, config: &ScriptConfig) -> Result<(), u8> {

        let mut title = String::new();

        for config_entry in config {
            match config_entry {
                // If the config entry contains a scene title, the title string is updated
                (true, text) => title = text.clone(),
                (false, text) => {
                    let mut fragment = SceneFragment::new(&title);
                    title = String::new();
                    if let Err(error_code) = fragment.prepare(text) {
                        return Err(error_code);
                    }
                    // Add the fragment to the play
                    self.fragments.push(fragment);
                }
            }
        }

        Ok(())

    }

    /// Processes individual lines:
    /// - Lines starting with [scene] are treated as scene titles
    /// - Other non-blank lines are treated as configuration filenames
    /// - Warns about missing scene titles or extra tokens (in whinge mode)
    fn add_config(line: &String, config: &mut ScriptConfig) {

        // Ignore blank lines
        if line.trim().is_empty() {
            return;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens[SCENE_SCRIPT_INDEX] == "[scene]" {
            // Case 1: [scene] title
            if tokens.len() < SCENE_SCRIPT_LENGTH && WHINGE_MODE.load(AtomicOrdering::SeqCst) {
                // Complain if no tokens apart from [scene] was provided
                eprintln!("Warning: [scene] without a scene title");
                return;
            } else {
                // Concatenate remaining tokens as the scene title if more tokens apart from [scene] were provided
                let scene_title = tokens[SCENE_TITLE_START_INDEX..].join(" ");
                config.push((true, scene_title))
            }
        } else {
            // Case 2: config filename
            let config_filename = tokens[CONFIG_FILENAME_INDEX].to_string();
            config.push((false, config_filename));
            // Complain if more tokens apart from the name of the configuration file were provided
            if tokens.len() > CONFIG_SCRIPT_LENGTH && WHINGE_MODE.load(AtomicOrdering::SeqCst) {
                let extra_tokens = tokens[CONFIG_EXTRA_TOKENS_START_INDEX..].join(" ");
                eprintln!("Warning: Extra tokens after configuration file name: '{}'", extra_tokens);
            }
        }

    }

    /// Parses the script file line-by-line into a ScriptConfig
    pub fn read_config(script_filename: &String, config: &mut ScriptConfig) -> Result<(), u8> {

        let mut script_lines: Vec<String> = Vec::new();

        if let Err(error_code) = grab_trimmed_file_lines(script_filename, &mut script_lines) {
            return Err(error_code);
        }

        if script_lines.is_empty() {
            eprintln!("Error: Script file '{}' contains no lines", script_filename);
            return Err(SCRIPT_PARSING_ERROR);
        }

        for line in &script_lines {
            Play::add_config(line, config);
        }

        Ok(())

    }

    /// Main entry point that:
    /// - Reads the script configuration file
    /// - Parses it into scene fragments
    /// - Validates that at least one fragment exists and the first has a title
    pub fn prepare(&mut self, script_filename: &String) -> Result<(), u8> {

        let mut config: ScriptConfig = Vec::new();

        if let Err(error_code) = Play::read_config(script_filename, &mut config) {
            return Err(error_code);
        }

        if let Err(error_code) = self.process_config(&config) {
            return Err(error_code);
        }

        if self.fragments.is_empty() {
            eprintln!("Error: No scene fragments were created");
            return Err(SCRIPT_PARSING_ERROR);
        }

        if !self.fragments[FIRST_SCENE_FRAGMENT].has_scene_title() {
            eprintln!("Error: First fragment must have a title");
            return Err(SCRIPT_PARSING_ERROR);
        }

        Ok(())

    }

    ///  Executes the play:
    /// - Handles player entrances 
    /// - Each fragment recites its lines
    /// - Handles player exits 
    pub fn recite(&mut self) {

        let num_fragments = self.fragments.len();
        let mut current_fragment_number = FIRST_SCENE_FRAGMENT;

        while current_fragment_number < num_fragments {

            if current_fragment_number == FIRST_SCENE_FRAGMENT {
                // All characters in the scene enter for the first fragment
                self.fragments[current_fragment_number].enter_all();
            } else {
                let previous_fragment = &self.fragments[current_fragment_number-SCENE_FRAGMENT_STEP];
                self.fragments[current_fragment_number].enter(previous_fragment)
            }

            self.fragments[current_fragment_number].recite();

            println!();

            if current_fragment_number == num_fragments-1 {
                // All characters in the scene exit for the final fragment
                self.fragments[current_fragment_number].exit_all();
            } else {
                let next_fragment = &self.fragments[current_fragment_number+SCENE_FRAGMENT_STEP];
                self.fragments[current_fragment_number].exit(next_fragment)
            }

            current_fragment_number += SCENE_FRAGMENT_STEP;

        }

    }

}
