#[derive(Clone, PartialEq)]
pub enum AppState {
    List,
    Edit,
}

impl Default for AppState {
    fn default() -> Self {
        Self::List
    }
}
