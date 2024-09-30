use crate::Orientation;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Layout<Item> {
    items: Vec<Item>,
    orientation: Orientation,
}

impl<Item> Layout<Item> {}
