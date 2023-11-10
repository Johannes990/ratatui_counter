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

impl Tui {
    // Construct new instance of 'Tui'
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initialize the terminal interface
    /// 
    /// it enables the raw mode and sets terminal properties
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode();
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // define a custom panic hook to reset the terminal properties
        // this way, you won't have your terminal messed up if an unexpected error happens
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor();
        self.terminal.clear();
        Ok(())
    }
}
