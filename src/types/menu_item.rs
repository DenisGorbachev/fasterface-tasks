use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Eq, PartialEq, Hash, Clone, Debug)]
pub struct MenuItem {
    #[new(into)]
    icon: char,
    #[new(into)]
    label: String,
}

pub fn item(icon: impl Into<char>, label: impl Into<String>) -> MenuItem {
    MenuItem::new(icon, label)
}
