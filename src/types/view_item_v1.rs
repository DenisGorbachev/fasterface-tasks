use derive_getters::Getters;
use derive_more::{From, Into};
use std::fs::DirEntry;
use std::path::Path;

#[derive(Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Clone, Debug)]
pub struct ViewItemV1 {
    /// Multiple names are required because the user might type a synonym (e.g. "remove" instead of "delete", or "push" / "add" / "append" instead of "insert")
    /// TODO: If the user types a synonym, the displayed name should change to the synonym (e.g. if the user types "remove", the item should be displayed as "Remove", not "Delete")
    names: Vec<String>,
}

impl From<DirEntry> for ViewItemV1 {
    fn from(entry: DirEntry) -> Self {
        let path_buf = entry.path();
        if path_buf.is_dir() {
            Self::from_dir_path(path_buf)
        } else {
            Self::from_file_path(path_buf)
        }
    }
}

impl ViewItemV1 {
    pub fn new<S: Into<String>>(names: impl IntoIterator<Item = S>) -> Self {
        Self {
            names: names.into_iter().map(Into::into).collect(),
        }
    }

    pub fn from_dir_path(dir: impl AsRef<Path>) -> Self {
        Self {
            names: vec![format!("{}/", dir.as_ref().display())],
        }
    }

    pub fn from_file_path(file: impl AsRef<Path>) -> Self {
        Self {
            names: vec![file.as_ref().display().to_string()],
        }
    }
}
