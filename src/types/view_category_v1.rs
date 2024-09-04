use crate::ViewItemV1;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use itertools::Itertools;
use std::fs::read_dir;
use std::io;
use std::path::Path;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct ViewCategoryV1 {
    #[new(into)]
    name: String,
    items: Vec<ViewItemV1>,
}

impl ViewCategoryV1 {
    pub fn from_dir_entries(dir: impl AsRef<Path>) -> io::Result<Self> {
        let items = read_dir(dir)?
            .map(|result| result.map(ViewItemV1::from))
            .try_collect()?;
        Ok(Self::new("Children", items))
    }

    pub fn dir_actions() -> Self {
        let new = ViewItemV1::new;
        Self::new("Actions", vec![new(["Create a file"]), new(["Delete"])])
    }

    pub fn from_path_parents(path: impl AsRef<Path>) -> Self {
        let ancestors = path.as_ref().ancestors();
        // skip the first ancestor because it is equal to the path itself
        let ancestors = ancestors.skip(1);
        let items = ancestors.map(ViewItemV1::from_dir_path).collect_vec();
        Self::new("Parents", items)
    }
}
