pub mod events;
pub mod keys;
use self::keys::Key;
pub enum InputEvent {
    Input(Key),
    Tick,
}
