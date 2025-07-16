// src/main.rs

mod ui;

fn main() {
    dioxus_desktop::launch::launch(ui::app, vec![], vec![]);
}
