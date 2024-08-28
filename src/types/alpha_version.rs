use crate::AlphaVersionV1;
use derive_more::From;

#[derive(From, Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Debug)]
pub enum AlphaVersion {
    TheAlphaVersionV1(AlphaVersionV1),
}
