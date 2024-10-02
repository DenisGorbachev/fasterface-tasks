use derive_more::From;
use std::path::PathBuf;

/// It's OK to add more variants to this enum. If the enum tag has a `usize` type, it can have 2^64 variants.
#[derive(From, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub enum Locator {
    #[default]
    None,
    ThePathBuf(PathBuf),
}

impl Locator {}
