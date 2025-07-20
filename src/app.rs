// src/app.rs

use dioxus::prelude::*;

use crate::tabs;

#[component]
pub fn App() -> Element {
    let current_tab = use_signal(|| 0usize);

    rsx! {
        div {
            style: "width: 100vw; height: 100vh; font-family: sans-serif; background: #f7f7f7;",
            div {
                style: "display: flex; border-bottom: 2px solid #ccc; background: #e3e3e3; padding: 0.5rem 1rem;",
                // <-- THIS BLOCK MUST BE WRAPPED IN CURLY BRACES, WITH A TRAILING COMMA!
                {tabs::TAB_LIST.iter().enumerate().map(|(i, tab)| {
                    let mut tab_signal = current_tab.clone();
                    let color = if tab_signal() == i { "#1976d2" } else { "#444" };
                    let border = if tab_signal() == i { "border-bottom: 3px solid #1976d2;" } else { "" };
                    rsx!(
                        button {
                            key: "{i}",
                            style: "color: {color}; background: none; border: none; padding: 0.75rem 1.5rem; font-weight: bold; font-size: 1.1rem; cursor: pointer; {border}; border-radius: 0.5rem 0.5rem 0 0; margin-right: 1rem;",
                            onclick: move |_| tab_signal.set(i),
                            "{tab.label()}"
                        }
                    )
                })}, // <-- CRITICAL TRAILING COMMA
            }
            div {
                style: "padding: 2.5rem; height: calc(100vh - 4rem);",
                { tabs::render_tab(current_tab()) }
            }
        }
    }
}
