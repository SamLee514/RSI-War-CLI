// mod input_loop;

use chrono::{ DateTime, Utc };
use tui::layout::Alignment;
use tui::layout::Constraint;
use tui::layout::Direction;
use tui::layout::Layout;
use tui::style::Color;
use tui::style::Modifier;
use tui::style::Style;
use tui::text::Span;
use tui::text::Spans;
use tui::widgets::Block;
use tui::widgets::BorderType;
use tui::widgets::Borders;
use tui::widgets::Paragraph;
use tui::widgets::Tabs;
use std::io;
use std::thread;
use std::time::Instant;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::IntoRawMode;
use termion::{event::Key, async_stdin};
use termion::input::TermRead;

// use termion::{ async_stdin, event, input::TermRead };
// use termion::event;
// use std::sync::mpsc;

struct User {
    user_name: String,
    password: String,
    high_score: u32,
    created_at: DateTime<Utc>
}

#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Classic,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Classic => 1,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_loop = async_stdin().keys();
    let stdout = io::stdout();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Classic Mode", "Twister Mode", "Custom Games", "Quit"];
    let mut active_menu_item = MenuItem::Home;

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(2),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);
            
            let menu = menu_titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(
                        first,
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::UNDERLINED),
                    ),
                    Span::styled(rest, Style::default().fg(Color::White)),
                ])
            })
            .collect();

            let tabs = Tabs::new(menu)
                .select(active_menu_item.into())
                .block(Block::default().title("Menu").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().fg(Color::Yellow))
                .divider(Span::raw("|"));

            rect.render_widget(tabs, chunks[0]);
            

            let copyright = Paragraph::new("RSI-War 2021 - made with <3 by Sambo")
                .style(Style::default().fg(Color::LightCyan))
                .alignment(Alignment::Center)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::White))
                        .title("Copyright")
                        .border_type(BorderType::Plain),
                );

            rect.render_widget(copyright, chunks[2]);
        });
    }
    Ok(())
}

// use std::sync::mpsc::channel;
// use std::thread;
// use std::time::Duration;
// fn main()  {
//     let (tx, rx) = channel();

//     thread::spawn(move || {
//         tx.send("Hello world!").unwrap();
//         thread::sleep(Duration::from_secs(2)); // block for two seconds
//         tx.send("Delayed for 2 seconds").unwrap();
//     });
    
//     println!("{}", rx.recv().unwrap()); // Received immediately
//     println!("Waiting...");
//     println!("{}", rx.recv().unwrap()); // Received after 2 seconds
// }

