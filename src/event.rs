use crossterm::event::{KeyEvent, MouseEvent};
use std::{sync::mpsc, thread}; // sync::mpsc is a 'Multiple Producer Single Consumer' channel

/// Terminal events
#[derive(Clone, Copy, Debug)]
pub enum Event {
    // Terminal tick
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    // Event sender channel
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    
    // Event receiver channel
    receiver: mpsc::Receiver<Event>,

    // event handler thread
    handler: thread::JoinHandle<()>,
}
