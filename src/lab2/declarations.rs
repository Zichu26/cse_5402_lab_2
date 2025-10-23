use std::sync::atomic::AtomicBool;

pub const MIN_ARGS: usize = 2;  // program_name config_file
pub const MAX_ARGS: usize = 3;  // program_name config_file WHINGE_MODE
pub const PROGRAM_NAME_INDEX: usize = 0;
pub const CONFIG_FILE_INDEX: usize = 1;
pub const VERBOSE_FLAG_INDEX: usize = 2;

// exit codes
pub const BAD_COMMAND_LINE_ERROR: u8 = 1;  
pub const SCRIPT_GENERATION_ERROR: u8 = 2;
pub const SUCCESS: u8 = 0;  

pub static WHINGE_MODE: AtomicBool = AtomicBool::new(false);


