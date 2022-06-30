use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

use crate::inputs::keys::Key;

use super::{
    actions::{QuitAction, TestAction},
    App, AppReturn,
};

#[derive(Clone, PartialEq)]
pub enum AppState {
    List,
    Edit,
}
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
pub struct ListState {
    actions: Vec<Box<dyn TestAction>>,
    config: ListConfig,
}
impl ListState {
    pub fn new() -> Box<Self> {
        let actions: Vec<Box<dyn TestAction>> = vec![Box::new(QuitAction::new())];
        let config = ListConfig { test: 1 };
        return Box::new(Self { actions, config });
    }
}
impl State for ListState {
    fn do_action(&mut self, key: Key) -> AppReturn {
        self.actions
            .iter_mut()
            .find(|action| action.keys().contains(&key))
            .unwrap()
            .execute()
    }
}
pub struct ListConfig {
    test: i32,
}

impl Default for AppState {
    fn default() -> Self {
        Self::List
    }
}
