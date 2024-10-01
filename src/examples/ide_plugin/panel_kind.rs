use strum::Display;

#[allow(dead_code)]
#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
#[non_exhaustive]
pub enum RustRoverPanelKind {
    Editor,
    /// File browser
    Explorer,
    Debugger,
    Fasterface,
}

impl RustRoverPanelKind {}
