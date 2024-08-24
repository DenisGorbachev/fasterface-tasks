use clap::Parser;
use fasterface_tasks::{Cli, Outcome};
use std::io::{stderr, stdout};

#[tokio::main]
async fn main() -> Outcome {
    Cli::parse().run(&mut stdout(), &mut stderr()).await
}
