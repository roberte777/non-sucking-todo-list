use std::io::Stdout;

use tui::Frame;

use crate::{
    app::{App, AppReturn, GlobalState},
    inputs::keys::Key,
};

use super::State;

pub struct EditingState {}
impl EditingState {
    pub fn new() -> Box<Self> {
        return Box::new(Self {});
    }
}
impl State for EditingState {
    fn do_action(&mut self, key: Key, app: &GlobalState) -> AppReturn {
        todo!()
    }
    fn draw(&self, rect: &mut Frame<tui::backend::CrosstermBackend<Stdout>>, app: &App) {
        todo!()
    }
}
