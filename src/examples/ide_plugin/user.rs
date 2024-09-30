use crate::OperatingSystem;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Eq, PartialEq, Clone, Debug)]
pub struct User {
    #[new(into)]
    name: String,
    #[new(into)]
    os: OperatingSystem,
}

impl User {}
