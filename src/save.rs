use structopt::StructOpt;
use termion::{self, event::Key, async_stdin};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::clear;
use std::{ thread, time::{ Duration, Instant } };
use std::io::{self, Read, Write};
use hhmmss::Hhmmss;

struct Game<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> {
    one_key: char,
    two_key: char,
    health: u16,
    stdout: W,
    stdin: R,
}

impl <R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Game<R, W> {

    fn countdown(&mut self) {
        let characters: [char; 9] = ['3', '.', '.', '2', '.', '.', '1', '.', '.'];
        for character in characters {
            write!(self.stdout, "{}", character);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(500));
        }
        println!("{}\rGO!", clear::CurrentLine);
        thread::sleep(Duration::from_millis(200));
    }

    fn start(&mut self) {
        let start_time = Instant::now();
        let mut p1_count: u16 = 0;
        let mut p2_count: u16 = 0;

        let mut p1_down: bool = false;
        let mut p2_down: bool = false;

        let start = Instant::now();
        let mut now;
        let mut end = false;
        // let mut clock;
        loop {
            let inp = self.stdin.next();
            // Print output by the millisecond
            now = start.elapsed();
            if let Some(Ok(c)) = inp {
                match c {
                    Key::Char('f') => p1_count += 1,
                    Key::Char('j') => p2_count += 1,
                    Key::Char('q') => break,
                    _ => {},
                };

                io::stdout().flush().unwrap();
                if p1_count == self.health {
                    writeln!(self.stdout, "\r{}Player 1 wins!", clear::CurrentLine);
                    end = true;
                } else if p2_count == self.health {
                    writeln!(self.stdout, "\r{}Player 2 wins!", clear::CurrentLine);
                    end = true;
                }
            }

            if now.as_micros() % 1000 == 0 {
                writeln!(self.stdout, "\r{}Player 1 count: {}\tPlayer 2 count: {}", clear::All, p1_count, p2_count);
                writeln!(self.stdout, "\r{}Clock: {}", clear::CurrentLine, now.hhmmssxxx());
            }
            if end { return };
            
        }
    }
}

fn get_player_keys() -> (char, char) {
    // TODO: change
    return ('f', 'j');
}

fn init<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write>(mut stdout: W, stdin: R) {
    // TODO: Add more params
    write!(stdout, "{}", clear::All).unwrap();

    let (player_one_key, player_two_key) = get_player_keys();

    // Set the initial game state
    let mut game = Game {
        one_key: player_one_key,
        two_key: player_two_key,
        health: 100,
        stdin: stdin,
        stdout: stdout,
    };

    game.countdown();
    game.start();
}

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = async_stdin().keys();

    init(stdout, stdin);
}
