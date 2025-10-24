/// main.rs
/// Author: Zichu Pan, Edgar Palomino
/// Summary: This is the entry point for the program. It handles command-line argument 
/// parsing and orchestrates the overall execution flow.
use std::env;
use std::sync::atomic::Ordering;

pub mod lab2;
use lab2::declarations::{MIN_ARGS, MAX_ARGS, PROGRAM_NAME_INDEX, CONFIG_FILE_INDEX, 
                         VERBOSE_FLAG_INDEX, BAD_COMMAND_LINE_ERROR, SUCCESS,
                         WHINGE_MODE};
use lab2::play::Play;
use lab2::return_wrapper::ReturnWrapper;

fn usage(program_name: &String) {
    println!("usage: {} <script_file_name> [whinge]", program_name);
}

fn parse_args(script_filename: &mut String) -> Result<(), u8> {
    let mut args: Vec<String> = Vec::new();
    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < MIN_ARGS || args.len() > MAX_ARGS {
        usage(&args[PROGRAM_NAME_INDEX]);
        return Err(BAD_COMMAND_LINE_ERROR);
    }

    if args.len() == MAX_ARGS && args[VERBOSE_FLAG_INDEX] != "whinge" {
        usage(&args[PROGRAM_NAME_INDEX]);
        return Err(BAD_COMMAND_LINE_ERROR);
    }

    *script_filename = args[CONFIG_FILE_INDEX].clone();

    if args.len() == MAX_ARGS && args[VERBOSE_FLAG_INDEX] == "whinge" {
        WHINGE_MODE.store(true, Ordering::SeqCst);
    }
    
    Ok(())
}
    
fn main() -> ReturnWrapper {
    let mut script_filename = String::new();

    if let Err(error_code) = parse_args(&mut script_filename) {
        return ReturnWrapper::new(error_code);
    }

    let mut play = Play::new();

    if let Err(error_code) = play.prepare(&script_filename) {
        return ReturnWrapper::new(error_code);
    }

    play.recite();

    ReturnWrapper::new(SUCCESS)
}
