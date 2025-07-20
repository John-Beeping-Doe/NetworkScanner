Certainly! Here’s an updated README tailored for a **Dioxus-based** Rust network scanner. This version removes all Ratatui/Crossterm references and sets clear expectations for cross-platform desktop **and** future web support, while retaining your modular, keyboard-driven focus.

---

# NetworkScanner

A **fast, cross-platform Rust network dashboard** built with [Dioxus](https://dioxuslabs.com/).
Get real-time network scanning and diagnostics in a modern UI, running natively on Linux, macOS, and Windows.

---

## Features

* **Tabbed dashboard**: Ping, Traceroute, DNS lookup, Port scanning, and more
* **Real-time results:** Live updating panels, history, and logs
* **Modern UI**: Mouse and keyboard navigation, responsive layout
* **Cross-platform**: Native desktop app (Dioxus Desktop) and future web support
* **Modular tools**: Add your own network features easily
* **Lightweight**: No Electron or JS runtime required

---

## Getting Started

```bash
# 1. Clone the repo
git clone https://github.com/John-Beeping-Doe/NetworkScanner.git
cd NetworkScanner

# 2. Ensure Rust is installed
rustc --version

# 3. Install Dioxus CLI (for running/debugging)
cargo install dioxus-cli

# 4. Run in development mode (hot reload)
dioxus serve

# 5. Build for release (native desktop binary)
cargo build --release
```

---

## Project Structure

```
src/
  main.rs        # Entry point, launches Dioxus desktop app
  app.rs         # App state, tab logic, network tasks
  network.rs     # Async network logic (ping, DNS, etc.)
  ui/            # Dioxus component modules (main UI, panels, logs, etc.)
  assets/        # Static assets (icons, etc. if needed)
```

---

## Extending

* Add new tool logic in `network.rs`
* Add new tabs/panels as Dioxus components in `src/ui/`
* UI is **entirely Dioxus** – no terminal TUI code
* All PRs, ideas, and feedback welcome!

---

## Sample src/main.rs (minimal, Dioxus desktop app)

```rust
// src/main.rs

use dioxus::prelude::*;
mod app;
fn main() {
    dioxus_desktop::launch(app::App);
}
```

---

**License:** MIT

---

Let me know if you want a sample Dioxus component, or want this README expanded for web, Linux packaging, or anything else.
