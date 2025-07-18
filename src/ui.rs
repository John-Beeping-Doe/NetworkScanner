// src/ui.rs

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph, Tabs};
use crate::Tab;
use crate::network::PingResult;

pub fn draw(f: &mut Frame, current_tab: usize, ping_result: Option<&PingResult>) {
    let area = f.area();

    // Layout: Tabs row (top), main content (rest)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Tabs bar
            Constraint::Min(0),    // Main content
        ])
        .split(area);

    // Tab labels
    let titles: Vec<_> = Tab::ALL.iter().map(|t| t.name()).collect();

    let tabs = Tabs::new(titles)
        .select(current_tab)
        .block(Block::default().title("NetworkScanner Dashboard - [←/→] Switch Tabs | [q] Quit").borders(Borders::ALL))
        .highlight_style(Style::default().bold().fg(Color::Yellow));

    f.render_widget(tabs, chunks[0]);

    // Panel content for each tab
    let content = match current_tab {
        0 => { // Ping
            let txt = if let Some(result) = ping_result {
                if result.ok {
                    format!("Ping to {} succeeded!\nRTT: {} ms", result.addr, result.rtt_ms.unwrap_or(0))
                } else {
                    format!("Ping to {} failed!\nError: {}", result.addr, result.err.as_deref().unwrap_or("Unknown"))
                }
            } else {
                "Pinging 8.8.8.8...".to_string()
            };
            Paragraph::new(txt).block(Block::default().borders(Borders::ALL).title("Ping"))
        }
        idx => {
            let title = Tab::ALL[idx].name();
            Paragraph::new(format!("This is the {title} panel."))
                .block(Block::default().borders(Borders::ALL).title(title))
        }
    };

    f.render_widget(content, chunks[1]);
}
