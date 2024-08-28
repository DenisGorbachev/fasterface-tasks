use crate::StoryName;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use url::Url;
use StoryName::*;

pub fn get_commands(story: StoryName) -> &'static [&'static str] {
    match story {
        InstallHomebrewPackage => &["install homebrew"],
        CreateGithubRepo => &["github", "search create new repo"],
        AddRustFunctionArgument => todo!(),
        PlaySpotifyPlaylist => todo!(),
    }
}

pub struct App {
    url: Url,
    // input: String,
}

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

impl App {
    pub const SCHEME: &'static str = "fasterface";

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
            item('', "Files"),
            item('󰖟', "Websites"),
            item('', "Settings"),
            item('󰋗', "Help"),
        ]
    }
}
