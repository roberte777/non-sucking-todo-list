use std::io::Stdout;

use tui::Frame;

use crate::inputs::keys::Key;

use super::{App, AppReturn, GlobalState, Transition};

pub mod editing_state;
pub mod global_state;
pub mod list_state;
pub use editing_state::EditingState;
pub use list_state::ListState;

pub struct StateReturn {
    pub transition: Option<Transition>,
    pub app_return: AppReturn,
}

pub trait State {
    fn do_action(&mut self, key: Key, global_state: &GlobalState) -> StateReturn;
    fn draw(&self, rect: &mut Frame<tui::backend::CrosstermBackend<Stdout>>, app: &App);
}
