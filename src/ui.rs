// src/ui.rs

use dioxus::prelude::*;

pub fn app() -> Element {
    rsx! {
        div {
            style: "display: flex; height: 100vh; font-family: sans-serif;",
            // Sidebar
            div {
                style: "width: 220px; background: #21222c; color: #fff; padding: 1.5rem;",
                h2 { "Tools" }
                ul {
                    li { "Ping" }
                    li { "Traceroute" }
                    li { "DNS Lookup" }
                    li { "Port Scan" }
                }
            }
            // Main Content Area
            div {
                style: "flex: 1; background: #f6f7fb; padding: 2rem;",
                h1 { "Network Scanner" }
                p { "Select a tool from the sidebar to get started." }
            }
        }
    }
}
