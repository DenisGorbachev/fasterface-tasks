use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum StoryName {
    InstallHomebrewPackage,
    CreateGithubRepo,
    AddRustFunctionArgument,
    PlaySpotifyPlaylist,
}

impl StoryName {}
