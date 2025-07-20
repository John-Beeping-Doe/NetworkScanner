// src/tabs/ping.rs

use dioxus::prelude::*;

#[component]
pub fn PingTab() -> Element {
    rsx! {
        div {
            style: "background: #fff; color: #222; border-radius: 14px; box-shadow: 0 4px 24px #0001; padding: 2.5rem; min-width: 320px; max-width: 640px; margin: auto;",
            h2 { style: "color: #1976d2;", "Ping Tool" }
            p { "This tab will show real-time ping results." }
        }
    }
}
