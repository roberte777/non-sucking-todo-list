use crate::inputs::keys::Key;

use super::{
    actions::{Action, Actions},
    AppReturn,
};

pub trait State {
    fn do_action(&mut self, key: Key) -> AppReturn;
}
pub struct EditingState {}
impl EditingState {
    pub fn new() -> Box<Self> {
        return Box::new(Self {});
    }
}
impl State for EditingState {
    fn do_action(&mut self, key: Key) -> AppReturn {
        todo!()
    }
}
//TODO: Too complicated for first app. Instead of doing this, just have
//a mapping of keys to action enum. Then, in do_action, do stuff based on that
//action enum. Simplify this later, don't do it first
pub struct ListState {
    actions: Actions,
}
impl ListState {
    pub fn new() -> Box<Self> {
        let actions = vec![Action::Quit].into();
        return Box::new(Self { actions });
    }
}
impl State for ListState {
    fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Exit,
            }
        } else {
            return AppReturn::Continue;
        }
    }
}
