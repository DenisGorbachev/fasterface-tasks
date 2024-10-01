use crate::{Orientation, Widget};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Debug)]
pub struct Container {
    children: Vec<Box<dyn Widget>>,
    orientation: Orientation,
}

impl Container {}

impl Widget for Container {
    fn draw(&self) {
        todo!()
    }
}
