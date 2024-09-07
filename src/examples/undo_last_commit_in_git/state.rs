use crate::Repo;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use indexmap::IndexMap;
use std::path::PathBuf;

#[derive(new, Getters, From, Into, Eq, PartialEq, Default, Clone, Debug)]
pub struct State {
    repos: IndexMap<PathBuf, Repo>,
}

impl State {}
