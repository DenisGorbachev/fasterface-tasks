use crate::User;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;

#[derive(new, Getters, From, Into, Eq, PartialEq, Default, Clone, Debug)]
pub struct State {
    users: Vec<User>,
}

impl State {}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn must_show_progress_for_long_running_tasks() {
        todo!()
    }

    #[test]
    #[ignore]
    fn must_detect_long_running_tasks_even_if_developer_did_not_mark_it_as_long_running_explicitly() {
        todo!()
    }
}
