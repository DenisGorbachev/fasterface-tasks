use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Language {
    Rust,
    Go,
    TypeScript,
    Lua,
    C,
    Cpp,
    Ruby,
}

pub use Language::*;

impl Language {}
