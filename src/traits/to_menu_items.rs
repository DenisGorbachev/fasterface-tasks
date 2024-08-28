use crate::MenuItem;

pub trait ToMenuItems {
    fn to_menu_items(&self) -> Vec<MenuItem>;
}
