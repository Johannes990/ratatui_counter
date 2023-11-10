use crossterm::event::{KeyEvent, MouseEvent};

/// Terminal events
#[derive(Clone, Copy, Debug)]
pub enum Event {
    // Terminal tick
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}