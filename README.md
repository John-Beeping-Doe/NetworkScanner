Here’s an **updated README.md** for your new Rust + Dioxus version of the NetworkScanner project.
This is clear, direct, and accurate for a Dioxus desktop or web app, and includes Rust-focused setup and instructions.

---

````markdown
# NetworkScanner

A modern dashboard for network scanning and diagnostics, built with **Rust** and **Dioxus**.

This project provides a cross-platform (Linux, macOS, Windows) GUI for running common network utilities (ping, traceroute, DNS lookup, and more), with results displayed in real time.

---

## Features

- Run network tools (ping, traceroute, DNS lookup, etc.) from the app
- Live output streaming and logs
- Modern, responsive UI with Dioxus (Rust UI framework)
- Modular architecture for easy addition of new tools
- Fast and secure: written entirely in Rust

---

## Getting Started

### Prerequisites

- **Rust toolchain** (rustc, cargo)
- [Dioxus CLI](https://dioxuslabs.com/guide/en/cli/)  
- Linux, macOS, or Windows

### Installation

#### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
````

#### 2. Install Build Tools (Linux)

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

#### 3. Install Dioxus CLI

```bash
cargo install dioxus-cli
```

#### 4. Clone the Repository

```bash
git clone https://github.com/John-Beeping-Doe/NetworkScanner-Rust.git
cd NetworkScanner-Rust/network_dashboard
```

#### 5. Run the App (Desktop)

```bash
dioxus serve --desktop
```

*For web target, use `dioxus serve --web` instead.*

---

## Project Structure

```
network_dashboard/
  ├── src/              # Rust source files
  ├── public/           # Static assets
  ├── Cargo.toml        # Rust project manifest
  ├── README.md
  ├── LICENSE
  └── .gitignore
```

---

## Contributing

Contributions welcome!
Please open an issue or submit a pull request for feature suggestions or bug fixes.

---

## License

This project is licensed under the MIT License.
See the [LICENSE](LICENSE) file for details.

---

## Author

Joshua Wood

```

---

**Let me know if you want any changes, want to specify Dioxus desktop vs web, or want a more detailed usage section!**
```
