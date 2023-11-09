use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::Paragraph,
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // startup: Enalbe raw mode for the terminal,  giving us fine control over
    // the user input
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

    // initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // define our counter variable
    // this is the state of our application
    let mut counter = 0;

    // main app loop
    loop {
        // render UI
        terminal.draw(|f| {
            f.render_widget(Paragraph::new(format!("Counter: {counter}")), f.size());
        })?;

        // check for user input every 250 ms
        if crossterm::event::poll(std::time::Duration::from_millis(250))? {
            // handle key events
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                
                // this speficic check for KeyEventKind::Press is here so that on windows
                // we wouldn't get 2 events for every press, because on windows we other-
                // wise would send the same event twice - on Press and on Release
                if key.kind == crossterm::event::KeyEventKind::Press {
                    match key.code {
                        crossterm::event::KeyCode::Char('j') => counter += 1,
                        crossterm::event::KeyCode::Char('k') => counter -= 1,
                        crossterm::event::KeyCode::Char('q') => break,
                        _ => {},
                    }
                }
            }
        }
    }

    // shutdown: reset terminal back to original state
    crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
