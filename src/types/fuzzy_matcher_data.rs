use crate::{FuzzyMatcherName, Language};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use url::Url;
use url_macro::url;

#[derive(new, Getters, From, Into, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub struct FuzzyMatcherData {
    url: Url,
    language: Language,
}

impl FuzzyMatcherData {}

use FuzzyMatcherName::*;
use Language::*;

impl From<FuzzyMatcherName> for FuzzyMatcherData {
    fn from(name: FuzzyMatcherName) -> Self {
        match name {
            Fzf => Self {
                url: url!("https://github.com/junegunn/fzf"),
                language: Go,
            },
            Nucleo => Self {
                url: url!("https://github.com/helix-editor/nucleo"),
                language: Rust,
            },
            Skim => Self {
                url: url!("https://github.com/lotabout/skim"),
                language: Rust,
            },
            Pick => Self {
                url: url!("https://github.com/mptre/pick"),
                language: C,
            },
            CommandT => Self {
                url: url!("https://github.com/wincent/command-t"),
                language: Lua,
            },
            Selecta => Self {
                url: url!("https://github.com/garybernhardt/selecta"),
                language: Ruby,
            },
            Fzy => Self {
                url: url!("https://github.com/jhawthorn/fzy"),
                language: C,
            },
            Peco => Self {
                url: url!("https://github.com/peco/peco"),
                language: Go,
            },
        }
    }
}
