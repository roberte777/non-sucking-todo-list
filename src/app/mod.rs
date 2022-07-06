use crate::{inputs::keys::Key, todo::ToDo};

use self::state::{global_state::GlobalState, EditingState, ListState, State};

pub mod actions;
pub mod state;
pub mod ui;
#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}
pub enum Transition {
    Editing,
    List,
}

pub struct App {
    global_state: GlobalState,
    state: Box<dyn State>,
}

impl App {
    pub fn new() -> Self {
        // for now it could be replaced with impl Default
        let state = ListState::new();
        let list = vec![
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
            ToDo {
                content: String::from("Testing!"),
            },
        ];
        let global_state = GlobalState { list };
        Self {
            state,
            global_state,
        }
    }

    pub fn do_action(&mut self, key: Key) -> AppReturn {
        let state_return = self.state.do_action(key, &mut self.global_state);
        if let Some(transition) = state_return.transition {
            self.transition(transition);
        }
        return state_return.app_return;
    }

    fn transition(&mut self, transition_to: Transition) {
        match transition_to {
            Transition::List => self.state = ListState::new(),
            Transition::Editing => self.state = EditingState::new(),
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
