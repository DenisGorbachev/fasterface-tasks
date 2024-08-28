use std::io::Write;

use crate::Outcome;
use crate::Ui;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub struct PrintCommand {}

impl PrintCommand {
    pub async fn run(self, _stdout: &mut impl Write, _stderr: &mut impl Write) -> Outcome {
        let _ui = Ui::choose();
        todo!()
    }
}
