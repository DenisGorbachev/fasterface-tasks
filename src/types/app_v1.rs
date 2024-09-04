use crate::{MenuItem, Outcome, ViewCategoryV1};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use std::io;
use std::path::PathBuf;
use url::Url;

#[derive(new, Getters, From, Into, Eq, PartialEq, Hash, Clone, Debug)]
pub struct AppV1 {
    /// The locator of the currently selected object
    url: Url,
    /// The user input in the search field
    input: String,
    /// The index of the selected item in the view area
    selected_index: usize,
    /// The home directory of the current user
    home_dir: PathBuf,
}

impl AppV1 {
    pub const SCHEME: &'static str = "fasterface";

    pub fn launch(url: impl Into<Url>, home_dir: impl Into<PathBuf>) -> Self {
        Self {
            url: url.into(),
            input: Default::default(),
            selected_index: Default::default(),
            home_dir: home_dir.into(),
        }
    }

    pub fn view(&self) -> Outcome<Vec<ViewCategoryV1>> {
        match self.url.host_str() {
            None => Ok(vec![]),
            Some("localhost") => match self.url.path_segments() {
                None => Ok(self.view_home_dir()?),
                Some(_segments) => todo!(),
            },
            Some(_host) => todo!(),
        }
    }

    pub fn view_home_dir(&self) -> io::Result<Vec<ViewCategoryV1>> {
        Ok(vec![
            ViewCategoryV1::from_dir_entries(self.home_dir.as_path())?,
            ViewCategoryV1::dir_actions(),
            ViewCategoryV1::from_path_parents(self.home_dir.as_path()),
        ])
    }

    pub fn is_own_url(&self) -> bool {
        self.url.scheme() == Self::SCHEME
    }

    pub fn menu_items(&self) -> Vec<MenuItem> {
        let host_opt = self.url.host_str();
        match host_opt {
            Some(_host) => {
                // let segments = self.url.path_segments();
                // match host {
                //     _ => ""
                // }
                // self.internal_menu_items(),
                todo!()
            }
            None => self.empty_menu_items(),
        }
    }

    pub fn internal_menu_items(&self) -> Vec<MenuItem> {
        match self.url.path_segments() {
            None => self.empty_menu_items(),
            Some(mut segments) => match segments.next() {
                Some(_segment) => todo!(),
                _ => self.empty_menu_items(),
            },
        }
    }

    pub fn empty_menu_items(&self) -> Vec<MenuItem> {
        vec![
            crate::item('', "Files"),
            crate::item('󰖟', "Websites"),
            crate::item('', "Settings"),
            crate::item('󰋗', "Help"),
        ]
    }
}
