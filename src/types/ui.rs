use derive_more::From;
use strum::EnumDiscriminants;

#[derive(From, EnumDiscriminants, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Ui {
    /// Use Amazon Q or another autocomplete UI
    /// Provide a separate `select` command that searches over multiple protocols
    /// Maybe even use fzf to search
    /// Allow other commands to read the `select.json` to see the selected objects
    /// Allow other commands to invoke `fzf` to get the option values (unless they are provided directly)
    /// Start the terminal in autocomplete mode
    TheCliWithAutocompleteUi,
    TheOnlineCliWithAutocompleteUi,
    TheTui,
    TheGui,
}

impl Ui {
    pub fn choose() -> Self {
        todo!()
    }
}

use UiDiscriminants::*;

impl UiDiscriminants {
    pub fn notes(self) -> &'static [&'static str] {
        match self {
            TheCliWithAutocompleteUi => &[
                "And allows the developers to build CLI tools that work regardless of whether our app is installed or not",
                "And doesn't require any code to implement",
                "And allows the user to utilize any autocomplete tool & customize it to their needs",
                "But requires the user to install and configure an autocomplete tool (we can ask the user to do it on the first launch)",
                "But doesn't allow us to show our token info",
                "But works only for command-line users (we can build a pre-configured web version - see TheOnlineCliWithAutocompleteUi)",
            ],
            TheOnlineCliWithAutocompleteUi => &["We can build an online terminal-like app with pre-configured autocomplete and everything"],
            TheTui => {
                todo!()
            }
            TheGui => {
                todo!()
            }
        }
    }
}
