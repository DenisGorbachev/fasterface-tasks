use crate::input::Input;
use crate::{Container, Panel, SupportText, Text, Vertical, Widget};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use fasterface::Context;

#[derive(new, Getters, From, Into, Debug)]
pub struct Fasterface {
    context: Context,
    support_text: SupportText,
}

impl Fasterface {
    pub fn to_panel(&self) -> Panel {
        let children: Vec<Box<dyn Widget>> = vec![
            Box::new(Input::new("")),
            Box::new(Text::new(self.support_text.clone())),
        ];
        let container = Container::new(children, Vertical);
        Panel::new("Fasterface", container)
    }
}
