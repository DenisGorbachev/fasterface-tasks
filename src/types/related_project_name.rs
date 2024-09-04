use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum RelatedProjectName {
    IntelliShell,
}

#[allow(dead_code)]
pub use RelatedProjectName::*;

impl RelatedProjectName {}
