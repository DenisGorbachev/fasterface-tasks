use strum::Display;

#[derive(Display, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum FuzzyMatcherName {
    Fzf,
    Nucleo,
    Skim,
    Pick,
    CommandT,
    Selecta,
    Fzy,
    Peco,
}

impl FuzzyMatcherName {}
