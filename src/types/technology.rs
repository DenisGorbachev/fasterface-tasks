use crate::KrateName;
use derive_more::From;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum Technology {
    Crate(KrateName),
}

impl Technology {}
