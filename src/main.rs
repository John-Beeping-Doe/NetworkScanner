// src/main.rs

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{error::Error, io, time::Duration};
use tokio::sync::mpsc;

mod network;
mod ui;

#[derive(Copy, Clone)]
pub enum Tab {
    Summary,
    Ping,
    Sweep,
    Dns,
    PortScan,
    Logs,
    Settings,
}
impl Tab {
    const ALL: [Tab; 7] = [
        Tab::Summary,
        Tab::Ping,
        Tab::Sweep,
        Tab::Dns,
        Tab::PortScan,
        Tab::Logs,
        Tab::Settings,
    ];
    fn name(&self) -> &'static str {
        match self {
            Tab::Summary => "Summary",
            Tab::Ping => "Ping",
            Tab::Sweep => "Sweep",
            Tab::Dns => "DNS",
            Tab::PortScan => "Port Scan",
            Tab::Logs => "Logs",
            Tab::Settings => "Settings",
        }
    }
}

pub struct AppState {
    pub current_tab: usize,
    pub targets: Vec<String>,
    pub ping_results: Vec<Option<network::PingResult>>,
    pub ping_history: Vec<Vec<network::PingResult>>,
    pub tests_enabled: Vec<[bool; 5]>,
    // For Settings tab navigation:
    pub settings_row: usize,    // Which target is selected
    pub settings_col: usize,    // Which column (service) is selected
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let targets = vec!["8.8.8.8".to_string(), "google.ca".to_string()];

    // Set up ping
    let (tx, mut rx) = mpsc::channel(32);
    for t in &targets {
        let tx_clone = tx.clone();
        let addr = t.clone();
        tokio::spawn(async move {
            network::ping_task(addr, tx_clone).await;
        });
    }

    let tests_enabled = vec![[true; 5]; targets.len()];

    let mut app = AppState {
        current_tab: 0,
        targets: targets.clone(),
        ping_results: vec![None; targets.len()],
        ping_history: vec![Vec::new(); targets.len()],
        tests_enabled,
        settings_row: 0,
        settings_col: 0,
    };

    'main: loop {
        // Receive ping results and update state
        while let Ok(result) = rx.try_recv() {
            if let Some(idx) = app.targets.iter().position(|t| t == &result.addr) {
                app.ping_results[idx] = Some(result.clone());
                let history = &mut app.ping_history[idx];
                history.push(result);
                if history.len() > 20 {
                    history.remove(0);
                }
            }
        }

        terminal.draw(|f| {
            ui::draw(f, &app);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Tab => {
                        app.current_tab = (app.current_tab + 1) % Tab::ALL.len();
                    }
                    KeyCode::Char('q') => break 'main,
                    _ => {
                        match app.current_tab {
                            6 => handle_settings_keys(&mut app, key), // Settings
                            _ => {} // other tabs: add arrow/list navigation as desired
                        }
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Handles navigation and toggling for the Settings tab
fn handle_settings_keys(app: &mut AppState, key: KeyEvent) {
    let num_targets = app.targets.len();
    let num_cols = 5;
    match key.code {
        KeyCode::Up => {
            if app.settings_row > 0 {
                app.settings_row -= 1;
            }
        }
        KeyCode::Down => {
            if app.settings_row + 1 < num_targets {
                app.settings_row += 1;
            }
        }
        KeyCode::Left => {
            if app.settings_col > 0 {
                app.settings_col -= 1;
            }
        }
        KeyCode::Right => {
            if app.settings_col + 1 < num_cols {
                app.settings_col += 1;
            }
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if app.settings_row < num_targets && app.settings_col < num_cols {
                app.tests_enabled[app.settings_row][app.settings_col] ^= true;
            }
        }
        _ => {}
    }
}
