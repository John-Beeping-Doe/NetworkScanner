// src/tabs/logs.rs

use dioxus::prelude::*;

#[component]
pub fn LogsTab() -> Element {
    rsx! {
        div {
            style: "background: #212432; border-radius: 12px; padding: 2rem;",
            h2 { "Logs" }
            p { "Logs and events will be displayed here." }
        }
    }
}
