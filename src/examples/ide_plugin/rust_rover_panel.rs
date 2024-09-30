use crate::RustRoverPanelKind;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct RustRoverPanel {
    #[getter(copy)]
    kind: RustRoverPanelKind,
    is_open: bool,
}

impl RustRoverPanel {
    pub fn new_default(kind: RustRoverPanelKind) -> Self {
        Self {
            kind,
            is_open: Default::default(),
        }
    }
}
