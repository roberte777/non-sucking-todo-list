use std::collections::HashMap;
use std::fmt::{self, Display};
use std::slice::Iter;

use crate::inputs::keys::Key;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum EditAction {
    Quit,
    List,
}

impl EditAction {
    /// All available actions
    pub fn iterator() -> Iter<'static, EditAction> {
        static ACTIONS: [EditAction; 2] = [EditAction::Quit, EditAction::List];
        ACTIONS.iter()
    }

    /// List of key associated to action
    pub fn keys(&self) -> &[Key] {
        match self {
            EditAction::Quit => &[Key::Ctrl('c')],
            EditAction::List => &[Key::Ctrl('s')],
        }
    }
}

/// Could display a user-friendly short description of action
impl Display for EditAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            EditAction::Quit => "Quit",
            EditAction::List => "Entering List Mode"
            // Action::Sleep => "Sleep",
            // Action::IncrementDelay => "Increment delay",
            // Action::DecrementDelay => "Decrement delay",
        };
        write!(f, "{}", str)
    }
}

#[derive(Default, Debug, Clone)]
pub struct EditActions(Vec<EditAction>);

impl EditActions {
    /// Given a key, find the corresponding action
    pub fn find(&self, key: Key) -> Option<&EditAction> {
        EditAction::iterator()
            // .filter(|action| self.0.contains(action))
            .find(|action| action.keys().contains(&key))
    }

    /// Get contextual actions.
    /// (just for building a help view)
    pub fn actions(&self) -> &[EditAction] {
        self.0.as_slice()
    }
}

impl From<Vec<EditAction>> for EditActions {
    fn from(actions: Vec<EditAction>) -> Self {
        // Check key unicity
        let mut map: HashMap<Key, Vec<EditAction>> = HashMap::new();
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
                    .map(EditAction::to_string)
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
