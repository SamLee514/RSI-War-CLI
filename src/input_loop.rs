use termion::event;
use std::sync::mpsc;
use std::thread;
use termion::input::TermRead;
use std::time::Instant;

// TODO: separate high-latency event loop that has ticks for processing regular input outside of game
enum Event<I> {
    Keydown(I),
    Tick,
}

pub struct Input {
    tx: mpsc::Sender<termion::event::Event>,
    rx: mpsc::Receiver<termion::event::Event>,
}

impl Input {
    pub fn new() -> Input {
        let (tx, rx) = mpsc::channel();
        Input { tx, rx }
    }

    pub fn run(&mut self, speedy: bool) {
        let input = async_stdin.keys();
        let tx = self.tx.clone();
        loop {
            match input.next() {
                Ok(key) => tx.send(Event::Keydown(key)).unwrap(),
                Err(error) => tx.send(Event::Tick).unwrap(),
            }
            if !speedy { 
                thread::sleep(Duration::from_millis(200));
            }
        }
    }
}
