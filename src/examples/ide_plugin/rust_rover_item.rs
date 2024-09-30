use derive_more::From;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum RustRoverItem {
    Text(String),
}

impl RustRoverItem {}
