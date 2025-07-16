// src/main.rs

use dioxus::prelude::*;
// use dioxus_desktop::launch;

static COLORS: &[&str] = &[
    "red", "green", "blue", "purple", "orange", "magenta", "teal", "maroon",
];

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut color_idx = use_signal(|| 0);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; height: 100vh;",
            h1 {
                style: "margin-bottom: 32px;",
                "Rust + Dioxus: Hello World!"
            }
            button {
                style: "padding: 16px 32px; font-size: 1.5rem; cursor: pointer; margin-bottom: 24px;",
                onclick: move |_| color_idx.set((color_idx() + 1) % COLORS.len()),
                "Change Color"
            }
            div {
                style: format_args!("font-size: 2rem; color: {};", COLORS[color_idx()]),
                "Hello World!"
            }
        }
    }
}
