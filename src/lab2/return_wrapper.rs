use std::process::{ExitCode, Termination};
use super::declarations::SUCCESS;

pub struct ReturnWrapper {
    code: u8,
}

impl ReturnWrapper {
    pub fn new(code: u8) -> ReturnWrapper {
        ReturnWrapper { code }
    }
}

impl Termination for ReturnWrapper {
    fn report(self) -> ExitCode {
        if self.code != SUCCESS {
            eprintln!("Error: {}", self.code);
        }
        ExitCode::from(self.code)
    }
}