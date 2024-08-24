use std::io::Write;

use clap::Parser;

use crate::outcome::Outcome;
use print::PrintCommand;
use Command::*;

pub mod print;

#[derive(Parser, Clone, Debug)]
pub enum Command {
    Print(PrintCommand),
}

impl Command {
    pub async fn run(self, stdout: &mut impl Write, stderr: &mut impl Write) -> Outcome {
        match self {
            Print(command) => command.run(stdout, stderr).await,
        }
    }
}
