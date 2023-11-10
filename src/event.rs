use crossterm::event::{
    self,
    KeyEvent,
    MouseEvent,
    Event as CrosstermEvent,
};

use anyhow::Result;

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
}; // sync::mpsc is a 'Multiple Producer Single Consumer' channel


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
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of 'EventHandler'
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel(); // here we create a channel for our sender and receiver
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or(tick_rate);

                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on windows
                                }
                            },
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failes to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self { sender, receiver, handler }
    }

    /// Receive the next event from the handler thread
    /// 
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
