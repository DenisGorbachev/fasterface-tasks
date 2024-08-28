use crate::{AlphaVersion, Design, KrateName, Outcome};

pub fn tui_first() -> Outcome<AlphaVersion> {
    let _design = design()?;
    let _crate_name = tui_crate_name()?;
    todo!()
}

pub fn design() -> Outcome<Design> {
    todo!()
}

pub fn tui_crate_name() -> Outcome<KrateName> {
    todo!()
}
