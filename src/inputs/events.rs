use std::{
    sync::mpsc::{channel, Receiver, RecvError, Sender},
    thread,
    time::Duration,
};

use super::{keys::Key, InputEvent};

pub struct Events {
    rx: Receiver<InputEvent>,
    // Need to be kept around to prevent disposing the sender side.
    _tx: Sender<InputEvent>,
}

impl Events {
    pub fn new(tick_rate: Duration) -> Events {
        let (tx, rx) = channel();

        let event_tx = tx.clone(); // the thread::spawn own event_tx
        thread::spawn(move || {
            loop {
                // poll for tick rate duration, if no event, sent tick event.
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                        let key = Key::from(key);
                        event_tx.send(InputEvent::Input(key)).unwrap();
                    }
                }
                event_tx.send(InputEvent::Tick).unwrap();
            }
        });

        Events { rx, _tx: tx }
    }

    /// Attempts to read an event.
    /// This function block the current thread.
    pub fn next(&self) -> Result<InputEvent, RecvError> {
        self.rx.recv()
    }
}
