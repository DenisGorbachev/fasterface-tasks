use strum::Display;
#[allow(dead_code)]
pub use RustRoverAction::*;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum RustRoverAction {
    TogglePanel,
}

impl RustRoverAction {}
