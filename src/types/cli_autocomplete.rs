use crate::StoryName;
use StoryName::*;

pub fn get_commands(story: StoryName) -> &'static [&'static str] {
    match story {
        InstallHomebrewPackage => &["install homebrew"],
        CreateGithubRepo => &["github", "search create new repo"],
        AddRustFunctionArgument => todo!(),
        PlaySpotifyPlaylist => todo!(),
    }
}
