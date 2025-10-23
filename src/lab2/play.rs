use std::sync::atomic::Ordering;
use super::scene_fragment::SceneFragment;
use super::declarations::{WHINGE_MODE, SCRIPT_GENERATION_ERROR};
use super::script_gen::grab_trimmed_file_lines;

pub type ScriptConfig = Vec<(bool, String)>;
pub type Fragments = Vec<SceneFragment>;

pub struct Play {
    fragments: Fragments,
}

impl Play {
    pub fn new() -> Play {
        Play {
            fragments: Vec::new(),
        }
    }

    pub fn process_config(&mut self, config: &ScriptConfig) -> Result<(), u8> {
        let mut title = String::new();
        
        for config_entry in config {
            match config_entry {
                (is_scene_title, text) => {
                    if *is_scene_title {
                        // Update the title string
                        title = text.clone();
                    } else {
                        let mut fragment = SceneFragment::new(&title);
                        title = String::new();
                        if let Err(_error_code) = fragment.prepare(text) {
                            return Err(SCRIPT_GENERATION_ERROR);
                        }
                        // Add the fragment to the play
                        self.fragments.push(fragment);
                    }
                }
            }
        }
        Ok(())
    }

    fn add_config(&mut self, config_line: &String, config: &mut ScriptConfig) {
        // Ignore blank lines
        if config_line.trim().is_empty() {
            return;
        }

        let tokens: Vec<&str> = config_line.split_whitespace().collect();

        // for token in &tokens {
        //     println!("{}", token);
        // }
        
        if tokens[0] == "[scene]" {
            if tokens.len() == 1 {
                // No scene title provided
                if WHINGE_MODE.load(Ordering::SeqCst) {
                    eprintln!("Warning: [scene]  without a scene title");
                }
                return;
            } else {
                // Concatenate remaining tokens as scene title
                let scene_title = tokens[1..].join(" ");
                config.push((true, scene_title));
            }
        } else {
            // First token is a configuration filename
            let config_file = tokens[0].to_string();
            config.push((false, config_file));
            
            if tokens.len() > 1 && WHINGE_MODE.load(Ordering::SeqCst) {
                eprintln!("Warning: Extra tokens after configuration file name: '{}'", tokens[1..].join(" "));
            }
        }
    }

    pub fn read_config(&mut self, script_filename: &String, config: &mut ScriptConfig) -> Result<(), u8> {
        let mut script_lines: Vec<String> = Vec::new();
        
        if let Err(_error_code) = grab_trimmed_file_lines(script_filename, &mut script_lines) {
            return Err(SCRIPT_GENERATION_ERROR);
        }

        if script_lines.is_empty() {
            eprintln!("Error: Script file '{}' contains no lines", script_filename);
            return Err(SCRIPT_GENERATION_ERROR);
        }
        
        for line in &script_lines {
            self.add_config(line, config);
        }
        
        Ok(())
    }

    pub fn prepare(&mut self, script_filename: &String) -> Result<(), u8> {
        let mut config: ScriptConfig = Vec::new();
        
        if let Err(_error_code) = self.read_config(script_filename, &mut config) {
            return Err(SCRIPT_GENERATION_ERROR);
        }
        
        if let Err(_error_code) = self.process_config(&config) {
            return Err(SCRIPT_GENERATION_ERROR);
        }

        if self.fragments.is_empty() {
            eprintln!("Error: No scene fragments were created");
            return Err(SCRIPT_GENERATION_ERROR);
        }
        
        if !self.fragments[0].has_title() {
            eprintln!("Error: First fragment must have a title");
            return Err(SCRIPT_GENERATION_ERROR);
        }
        
        Ok(())
    }

    pub fn recite(&mut self) {
        let num_fragments = self.fragments.len();
        
        for i in 0..num_fragments {
            if i == 0 {
                self.fragments[i].enter_all();
            } else {
                self.fragments[i].enter(&self.fragments[i - 1]);
            }
            
            self.fragments[i].recite();
            
            if i == num_fragments - 1 {
                self.fragments[i].exit_all();
            } else {
                self.fragments[i].exit(&self.fragments[i + 1]);
            }
        }
    }

}