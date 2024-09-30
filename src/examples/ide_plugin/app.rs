use crate::Ide;
use derive_more::From;

#[derive(From, Eq, PartialEq, Clone, Debug)]
pub enum App {
    TheIde(Ide),
}

impl App {}
