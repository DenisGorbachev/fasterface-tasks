use crate::RelatedProjectName;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use url::Url;
use url_macro::url;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct RelatedProjectData {
    url: Url,
}

use RelatedProjectName::*;

impl From<RelatedProjectName> for RelatedProjectData {
    fn from(name: RelatedProjectName) -> Self {
        match name {
            IntelliShell => Self {
                url: url!("https://github.com/lasantosr/intelli-shell"),
            },
        }
    }
}
