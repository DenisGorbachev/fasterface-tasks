use crate::MenuItem;
use derive_more::{Display, Error};
use url::Url;

pub trait TryToMenuItems {
    type Error;

    fn to_menu_items(&self) -> Result<Vec<MenuItem>, Self::Error>;
}

impl TryToMenuItems for Url {
    type Error = ();

    fn to_menu_items(&self) -> Result<Vec<MenuItem>, Self::Error> {
        todo!()
    }
}

#[derive(Error, Display, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct IncorrectSchemeError;
