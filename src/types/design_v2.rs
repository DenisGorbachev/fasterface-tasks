use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Hash, Clone, Debug)]
pub struct DesignV2 {
    files: Vec<syn::File>,
}

impl DesignV2 {}
