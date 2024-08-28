use crate::{KrateInfo, KrateName};
use derive_getters::{Dissolve, Getters};
use derive_new::new;

#[derive(new, Getters, Dissolve, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Krate {
    name: KrateName,
    info: KrateInfo,
}

impl Krate {}
