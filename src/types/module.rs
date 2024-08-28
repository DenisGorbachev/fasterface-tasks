use derive_more::From;
use strum::EnumDiscriminants;

#[derive(From, EnumDiscriminants, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum Module {
    TheFilesystemModule,
    TheSpotifyModule,
    TheGithubModule,
    TheRustIdeModule,
    TheLlmPromptModule,
    TheStripeModule,
}

impl Module {}

impl ModuleDiscriminants {
    pub fn choose() -> Self {
        // It's necessary to implement some functionality that is not present in other apps
        // This leaves us with RustIde code actions
        todo!()
    }
}
