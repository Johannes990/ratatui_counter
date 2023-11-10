use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};

use anyhow::Result;
pub type Frame<'a> = ratatui::Frame<'a>;

/// declare our files as modules
/// Application
pub mod app;

/// Terminal events handler
pub mod event;

/// Widget renderer
pub mod ui;

/// Terminal user interface
pub mod tui;

/// Application updater
pub mod update;

struct App {
    counter: i64,
    should_quit: bool,
}


fn startup() -> Result<()> {
    enable_raw_mode();
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode();
    Ok(())
}

fn ui(app: &App, f: &mut Frame<'_>) {
    f.render_widget(Paragraph::new(format!("Counter: {}", app.counter)), f.size());
}

fn update(app: &mut App) ->  Result<()> {
    if event::poll(std::time::Duration::from_millis(10000))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {},
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    // ratatui terminal
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // application state
    let mut app = App { counter: 0, should_quit: false };

    loop {
        t.draw(|f| {
            ui(&app, f);
        })?;

        // application update
        update(&mut app)?;

        // application exit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}


fn main() -> Result<()> {
    startup()?;
    let result = run()?;
    shutdown()?;
    result;
    Ok(())
}
