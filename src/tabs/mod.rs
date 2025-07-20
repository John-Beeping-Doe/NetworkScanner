// src/tabs/mod.rs

use dioxus::prelude::*;

pub mod ping;
pub mod traceroute;
pub mod sweep;
pub mod dns;
pub mod portscan;
pub mod logs;
pub mod settings;

#[derive(Clone, Copy, Debug)]
pub enum Tab {
    Ping,
    Traceroute,
    Sweep,
    Dns,
    PortScan,
    Logs,
    Settings,
}

pub const TAB_LIST: &[Tab] = &[
    Tab::Ping,
    Tab::Traceroute,
    Tab::Sweep,
    Tab::Dns,
    Tab::PortScan,
    Tab::Logs,
    Tab::Settings,
];

impl Tab {
    pub fn label(self) -> &'static str {
        match self {
            Tab::Ping => "Ping",
            Tab::Traceroute => "Traceroute",
            Tab::Sweep => "Sweep",
            Tab::Dns => "DNS",
            Tab::PortScan => "Port Scan",
            Tab::Logs => "Logs",
            Tab::Settings => "Settings",
        }
    }
}

pub fn render_tab(idx: usize) -> Element {
    match TAB_LIST.get(idx) {
        Some(Tab::Ping) => ping::PingTab(),
        Some(Tab::Traceroute) => traceroute::TracerouteTab(),
        Some(Tab::Sweep) => sweep::SweepTab(),
        Some(Tab::Dns) => dns::DnsTab(),
        Some(Tab::PortScan) => portscan::PortScanTab(),
        Some(Tab::Logs) => logs::LogsTab(),
        Some(Tab::Settings) => settings::SettingsTab(),
        _ => rsx! { div { "Unknown tab" } },
    }
}
