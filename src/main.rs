// src/main.rs

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
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
    Ping,
    Sweep,
    Dns,
    PortScan,
    Logs,
    Settings,
}
impl Tab {
    const ALL: [Tab; 6] = [
        Tab::Ping,
        Tab::Sweep,
        Tab::Dns,
        Tab::PortScan,
        Tab::Logs,
        Tab::Settings,
    ];
    fn name(&self) -> &'static str {
        match self {
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
    pub selected_target: usize,
    pub adding_target: bool,
    pub new_target: String,
    pub tests_enabled: [bool; 5], // Ping, Sweep, DNS, PortScan, Logs
    pub settings_selected: usize,
    pub ping_results: Vec<Option<network::PingResult>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set up ping
    let (tx, mut rx) = mpsc::channel(32);
    let targets = vec!["8.8.8.8".to_string()];
    for t in &targets {
        let tx_clone = tx.clone();
        let addr = t.clone();
        tokio::spawn(async move {
            network::ping_task(addr, tx_clone).await;
        });
    }

    let mut app = AppState {
        current_tab: 0,
        targets: targets.clone(),
        selected_target: 0,
        adding_target: false,
        new_target: String::new(),
        tests_enabled: [true, false, false, false, false],
        settings_selected: 0,
        ping_results: vec![None; targets.len()],
    };

    'main: loop {
        // Receive ping results and update state
        while let Ok(result) = rx.try_recv() {
            if let Some(idx) = app.targets.iter().position(|t| t == &result.addr) {
                app.ping_results[idx] = Some(result);
            }
        }

        terminal.draw(|f| {
            ui::draw(f, &app);
        })?;

        // Handle input
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app.current_tab == 5 {
                    let num_settings = app.targets.len() + 5;
                    if app.adding_target {
                        match key.code {
                            KeyCode::Esc => {
                                app.adding_target = false;
                                app.new_target.clear();
                            }
                            KeyCode::Enter => {
                                if !app.new_target.trim().is_empty() {
                                    let new_t = app.new_target.trim().to_string();
                                    app.targets.push(new_t.clone());
                                    app.ping_results.push(None);

                                    // Start ping for new target
                                    let tx_clone = tx.clone();
                                    tokio::spawn(network::ping_task(new_t, tx_clone));
                                }
                                app.adding_target = false;
                                app.new_target.clear();
                            }
                            KeyCode::Backspace => {
                                app.new_target.pop();
                            }
                            KeyCode::Char(c) => {
                                app.new_target.push(c);
                            }
                            _ => {}
                        }
                    } else {
                        match key.code {
                            KeyCode::Up => {
                                if app.settings_selected > 0 {
                                    app.settings_selected -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if app.settings_selected + 1 < num_settings {
                                    app.settings_selected += 1;
                                }
                            }
                            KeyCode::Char('a') => {
                                if app.settings_selected <= app.targets.len() {
                                    app.adding_target = true;
                                    app.new_target.clear();
                                }
                            }
                            KeyCode::Char('d') => {
                                if app.settings_selected < app.targets.len() && !app.targets.is_empty() {
                                    app.targets.remove(app.settings_selected);
                                    app.ping_results.remove(app.settings_selected);
                                    if app.settings_selected > 0 {
                                        app.settings_selected -= 1;
                                    }
                                }
                            }
                            KeyCode::Char(' ') => {
                                let idx = app.settings_selected;
                                if idx >= app.targets.len() && idx < app.targets.len() + 5 {
                                    let test_idx = idx - app.targets.len();
                                    app.tests_enabled[test_idx] = !app.tests_enabled[test_idx];
                                }
                            }
                            KeyCode::Char('q') => break 'main,
                            KeyCode::Right => app.current_tab = (app.current_tab + 1) % Tab::ALL.len(),
                            KeyCode::Left => {
                                if app.current_tab == 0 {
                                    app.current_tab = Tab::ALL.len() - 1;
                                } else {
                                    app.current_tab -= 1;
                                }
                            }
                            KeyCode::Char('1') => app.current_tab = 0,
                            KeyCode::Char('2') => app.current_tab = 1,
                            KeyCode::Char('3') => app.current_tab = 2,
                            KeyCode::Char('4') => app.current_tab = 3,
                            KeyCode::Char('5') => app.current_tab = 4,
                            KeyCode::Char('6') => app.current_tab = 5,
                            _ => {}
                        }
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => break 'main,
                        KeyCode::Right => app.current_tab = (app.current_tab + 1) % Tab::ALL.len(),
                        KeyCode::Left => {
                            if app.current_tab == 0 {
                                app.current_tab = Tab::ALL.len() - 1;
                            } else {
                                app.current_tab -= 1;
                            }
                        }
                        KeyCode::Char('1') => app.current_tab = 0,
                        KeyCode::Char('2') => app.current_tab = 1,
                        KeyCode::Char('3') => app.current_tab = 2,
                        KeyCode::Char('4') => app.current_tab = 3,
                        KeyCode::Char('5') => app.current_tab = 4,
                        KeyCode::Char('6') => app.current_tab = 5,
                        KeyCode::Up => {
                            if app.selected_target > 0 {
                                app.selected_target -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.selected_target + 1 < app.targets.len() {
                                app.selected_target += 1;
                            }
                        }
                        _ => {}
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
