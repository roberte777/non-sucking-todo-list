use crate::{inputs::keys::Key, todo::ToDo};

use self::{
    actions::{Action, Actions},
    state::AppState,
};

pub mod actions;
pub mod state;
pub mod ui;
#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    list: Vec<ToDo>,
    /// Contextual actions
    actions: Actions,
    /// State
    state: AppState,
}

impl App {
    pub fn new() -> Self {
        // for now it could be replaced with impl Default
        let actions = vec![Action::Quit].into();
        let state = AppState::List;
        let list = vec![ToDo {
            content: String::from("Testing!"),
        }];
        Self {
            actions,
            state,
            list,
        }
    }

    /// Handle a user action
    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Exit,
            }
        } else {
            AppReturn::Continue
        }
    }

    /// We could update the app or dispatch event on tick
    pub fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        AppReturn::Continue
    }
    pub fn update_state() {}

    // ...
}
