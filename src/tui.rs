use std::{io, panic};

use anyhow::Result;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::App, event::EventHandler, ui};


/// Representation of the terminal user interface
/// 
/// It it responsible for setting up and handling the draw events
/// intializing the interface and handling the draw events
pub struct Tui {
    terminal: CrosstermTerminal,
    pub events: EventHandler,
}
