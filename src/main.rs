// src/main.rs

/*
src/main.rs
*/

use ratatui::prelude::*;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, time::Duration};
use tokio::sync::mpsc;
use tokio::runtime::Runtime;

mod ui;
mod network;

#[derive(Copy, Clone)]
pub enum Tab {
    Ping,
    Sweep,
    Dns,
    PortScan,
    Logs,
}

impl Tab {
    const ALL: [Tab; 5] = [Tab::Ping, Tab::Sweep, Tab::Dns, Tab::PortScan, Tab::Logs];
    fn name(&self) -> &'static str {
        match self {
            Tab::Ping => "Ping",
            Tab::Sweep => "Sweep",
            Tab::Dns => "DNS",
            Tab::PortScan => "Port Scan",
            Tab::Logs => "Logs",
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Async runtime for ping
    let rt = Runtime::new()?;

    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // State: selected tab & ping result
    let mut current_tab = 0;
    let mut ping_result = None;

    // Ping communication channel
    let (ping_tx, mut ping_rx) = mpsc::channel(1);

    // Start ping task for 8.8.8.8
    rt.spawn(network::ping_task("8.8.8.8".to_string(), ping_tx));

    loop {
        // Try to get the latest ping result if any (non-blocking)
        while let Ok(res) = ping_rx.try_recv() {
            ping_result = Some(res);
        }

        terminal.draw(|f| {
            ui::draw(f, current_tab, ping_result.as_ref());
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => {
                        current_tab = (current_tab + 1) % Tab::ALL.len();
                    }
                    KeyCode::Left => {
                        if current_tab == 0 {
                            current_tab = Tab::ALL.len() - 1;
                        } else {
                            current_tab -= 1;
                        }
                    }
                    KeyCode::Char('1') => current_tab = 0,
                    KeyCode::Char('2') => current_tab = 1,
                    KeyCode::Char('3') => current_tab = 2,
                    KeyCode::Char('4') => current_tab = 3,
                    KeyCode::Char('5') => current_tab = 4,
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
