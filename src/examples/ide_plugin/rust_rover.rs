use crate::{Outcome, RustRoverAction, RustRoverPanel, WithFocus};
use crossterm::event::KeyEvent;
use derive_getters::Getters;
use derive_more::{From, Into};
use derive_new::new;
use indexmap::IndexMap;
use not_found_error::Require;
use RustRoverAction::*;

#[derive(new, Getters, From, Into, Eq, PartialEq, Clone, Debug)]
pub struct RustRover {
    shortcuts: IndexMap<KeyEvent, RustRoverAction>,
    panels: WithFocus<RustRoverPanel>,
}

impl RustRover {
    pub fn process_key_event(&mut self, event: &KeyEvent) -> Outcome {
        let action = self.shortcuts.get(event).cloned().require()?;
        self.process_action(&action)
    }

    pub fn process_action(&mut self, action: &RustRoverAction) -> Outcome {
        match action {
            TogglePanel => todo!(),
        }
    }

    pub fn panel(&self) -> Option<&RustRoverPanel> {
        self.panels.current()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Outcome, RustRover, RustRoverPanel, RustRoverPanelKind};
    use indexmap::IndexMap;
    use stub_macro::stub;
    use RustRoverPanelKind::*;

    #[test]
    #[ignore]
    fn must_show_fasterface_pane_when_user_hits_a_keyboard_shortcut() -> Outcome {
        let shortcut_event = stub!(KeyEvent);
        let new = RustRoverPanel::new_default;
        let panels_raw = vec![new(Explorer), new(Fasterface)];
        let panels = WithFocus::new(panels_raw, None);
        let shortcuts = IndexMap::from([(shortcut_event, TogglePanel)]);
        let mut rust_rover = RustRover::new(shortcuts, panels);
        rust_rover.process_key_event(&shortcut_event)?;
        let panel = rust_rover.panel().expect("at least one panel must exist");
        assert_eq!(panel.kind(), Fasterface);
        assert!(panel.is_open());
        Ok(())
    }

    #[test]
    #[ignore]
    fn fasterface_panel_must_have_a_link_to_the_token_at_the_bottom() {
        let _rust_rover = stub!(RustRover);
        todo!()
    }
}
