use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

use crate::inputs::keys::Key;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ListAction {
    Quit,
    SelectNext,
    SelectPrev,
    Edit,
}

impl ListAction {
    /// All available actions
    pub fn iterator() -> Iter<'static, ListAction> {
        static ACTIONS: [ListAction; 4] = [
            ListAction::Quit,
            ListAction::SelectNext,
            ListAction::SelectPrev,
            ListAction::Edit,
        ];
        ACTIONS.iter()
    }

    /// List of key associated to action
    pub fn keys(&self) -> &[Key] {
        match self {
            ListAction::Quit => &[Key::Ctrl('c'), Key::Char('q')],
            ListAction::SelectNext => &[Key::Char('j')],
            ListAction::SelectPrev => &[Key::Char('k')],
            ListAction::Edit => &[Key::Enter],
        }
    }
}

/// Could display a user-friendly short description of action
impl Display for ListAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            ListAction::Quit => "Quit",
            ListAction::SelectNext => "Selecting Next ToDo",
            ListAction::SelectPrev => "Selecting Previous ToDo",
            ListAction::Edit => "Editing ToDo"
            // Action::Sleep => "Sleep",
            // Action::IncrementDelay => "Increment delay",
            // Action::DecrementDelay => "Decrement delay",
        };
        write!(f, "{}", str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct ListActions(Vec<ListAction>);

impl ListActions {
    /// Given a key, find the corresponding action
    pub fn find(&self, key: Key) -> Option<&ListAction> {
        ListAction::iterator()
            // .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    /// Get contextual actions.
    /// (just for building a help view)
    pub fn actions(&self) -> &[ListAction] {
        self.0.as_slice()
    }
}

impl From<Vec<ListAction>> for ListActions {
    fn from(actions: Vec<ListAction>) -> Self {
        // Check key unicity
        let mut map: HashMap<Key, Vec<ListAction>> = HashMap::new();
        for action in actions.iter() {
            for key in action.keys().iter() {
                match map.get_mut(key) {
                    Some(vec) => vec.push(*action),
                    None => {
                        map.insert(*key, vec![*action]);
                    }
                }
            }
        }
        let errors = map
            .iter()
            .filter(|(_, actions)| actions.len() > 1) // at least two actions share same shortcut
            .map(|(key, actions)| {
                let actions = actions
                    .iter()
                    .map(ListAction::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Conflict key {} with actions {}", key, actions)
            })
            .collect::<Vec<_>>();
        if !errors.is_empty() {
            panic!("{}", errors.join("; "))
        }

        // Ok, we can create contextual actions
        Self(actions)
    }
}
