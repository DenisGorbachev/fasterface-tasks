use crate::App;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Eq, PartialEq, Clone, Debug)]
pub struct OperatingSystem {
    /// The currently focused app
    app: App,
}

impl OperatingSystem {}
