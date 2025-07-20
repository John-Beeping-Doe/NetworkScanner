// src/main.rs

mod tabs;
mod app;

use dioxus_desktop::launch::launch;

fn main() {
    launch(app::App, vec![], Default::default());
}
