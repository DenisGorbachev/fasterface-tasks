use crate::{CommitData, ObjectId};
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use indexmap::IndexMap;

#[derive(new, Getters, From, Into, Eq, PartialEq, Default, Clone, Debug)]
pub struct Repo {
    commits: IndexMap<ObjectId, CommitData>,
}

impl Repo {}
