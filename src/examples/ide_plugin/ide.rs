use crate::RustRover;
use derive_more::From;

#[derive(From, Eq, PartialEq, Clone, Debug)]
pub enum Ide {
    TheRustRover(RustRover),
}

impl Ide {}
