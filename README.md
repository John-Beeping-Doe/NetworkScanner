# NetworkScanner

A fast, cross-platform TUI network dashboard in Rust using [Ratatui](https://github.com/ratatui-org/ratatui) and [Crossterm](https://github.com/crossterm-rs/crossterm).

**Features:**
- Real-time ping, DNS, port scan, etc. (modular tools)
- Fully keyboard-driven terminal dashboard
- Lightweight, no desktop UI bloat
- Works on Linux, macOS, Windows

## Getting Started

```bash
# 1. Clone the repo
git clone https://github.com/John-Beeping-Doe/NetworkScanner.git
cd NetworkScanner

# 2. Ensure Rust is installed
rustc --version

# 3. Build and run in debug mode
cargo run

# 4. Build release
cargo build --release
Project Structure

src/
    main.rs      # Entry point & TUI event loop
    ui.rs        # Ratatui layout and rendering
    network.rs   # Async network scan logic (ping, DNS, etc.)
Extending

Add your tool logic in network.rs
Add new panels/sections in ui.rs
All TUI code is based on Ratatui and Crossterm
PRs and feedback welcome!


---

## 5. **Sample src/main.rs** (minimal working “hello TUI”)

```rust
// src/main.rs

use ratatui::prelude::*;
use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use std::io::{self, stdout};
use std::time::Duration;

mod ui;
mod network;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui::draw(f))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
