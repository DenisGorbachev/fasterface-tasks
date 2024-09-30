use crate::{Layout, Orientation};
use crate::{RustRoverItem, SupportText};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use Orientation::*;
use RustRoverItem::*;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct Panel<Item> {
    name: String,
    layout: Layout<Item>,
}

impl Panel<RustRoverItem> {
    pub fn fasterface(text: SupportText) -> Self {
        let items = vec![Text(text.into())];
        let layout = Layout::new(items, Vertical);
        Self {
            name: "Fasterface".to_string(),
            layout,
        }
    }
}
