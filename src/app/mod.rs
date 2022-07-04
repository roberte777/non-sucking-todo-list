use crate::todo::ToDo;

use self::state::{global_state::GlobalState, ListState, State};

pub mod actions;
pub mod state;
pub mod ui;
#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    global_state: GlobalState,
    /// State
    state: Box<dyn State>,
}

impl App {
    pub fn new() -> Self {
        // for now it could be replaced with impl Default
        let state = ListState::new();
        let list = vec![ToDo {
            content: String::from("Testing!"),
        }];
        let global_state = GlobalState { list };
        Self {
            state,
            global_state,
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
