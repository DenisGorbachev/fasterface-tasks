use crate::{DesignV1, DesignV2};
use derive_more::From;

#[derive(From, Hash, Clone, Debug)]
pub enum Design {
    TheDesignV1(DesignV1),
    TheDesignV2(DesignV2),
}

impl Design {}
