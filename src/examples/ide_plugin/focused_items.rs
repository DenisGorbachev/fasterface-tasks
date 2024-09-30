use derive_getters::Getters;
use derive_more::{Deref, From, Into};
use std::ops::Rem;

#[derive(Deref, Getters, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct WithFocus<Item> {
    #[deref]
    items: Vec<Item>,
    focus: Option<usize>,
}

impl<Item> WithFocus<Item> {
    pub fn new(items: Vec<Item>, focus: Option<usize>) -> Self {
        let focus = Self::get_focus_static(focus, items.len());
        Self {
            items,
            focus,
        }
    }

    pub fn set_focus(&mut self, focus: Option<usize>) {
        self.focus = Self::get_focus_static(focus, self.items.len())
    }

    pub fn move_focus(&mut self, increment: isize) {
        self.set_focus(self.focus.map(|index| index.wrapping_add_signed(increment)))
    }

    pub fn get_focus_static(focus: Option<usize>, len: usize) -> Option<usize> {
        focus.and_then(|index| match len {
            0 => None,
            non_zero_len => Some(index.rem(non_zero_len)),
        })
    }

    pub fn update_items<Out>(&mut self, update: impl FnOnce(&mut Vec<Item>) -> Out) -> Out {
        let out = update(&mut self.items);
        self.set_focus(self.focus);
        out
    }

    /// Safety: `index` is guaranteed to be in bounds
    pub fn current(&self) -> Option<&Item> {
        self.focus
            .map(|index| unsafe { self.items.get_unchecked(index) })
    }
}

impl<Item> From<Vec<Item>> for WithFocus<Item> {
    fn from(items: Vec<Item>) -> Self {
        Self {
            items,
            focus: None,
        }
    }
}
