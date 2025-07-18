// src/ui/mod.rs

use ratatui::prelude::*;
use crate::{AppState, Tab};

pub mod ping;
pub mod summary;
pub mod settings; // <-- ADD THIS!

pub fn draw(f: &mut Frame, app: &AppState) {
    let chunks_main = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(f.area());

    // Tabs at the top
    let tab_titles: Vec<_> = Tab::ALL.iter().map(|t| t.name()).collect();
    let tabs = ratatui::widgets::Tabs::new(tab_titles)
        .select(app.current_tab)
        .block(
            ratatui::widgets::Block::default()
                .title("NetworkScanner Dashboard – [←/→] Tabs [q] Quit")
                .borders(ratatui::widgets::Borders::ALL),
        )
        .highlight_style(ratatui::style::Style::default().bold().fg(ratatui::style::Color::Yellow));
    f.render_widget(tabs, chunks_main[0]);

    // Tab panels
    match app.current_tab {
        0 => summary::draw_summary_panel(f, chunks_main[1], app),
        1 => ping::draw_ping_panel(f, chunks_main[1], app),
        6 => settings::draw_settings_panel(f, chunks_main[1], app), // <<== THIS LINE CALLS THE REAL SETTINGS PANEL!
        _ => {
            let title = Tab::ALL[app.current_tab].name();
            let para = ratatui::widgets::Paragraph::new(format!("This is the {title} panel."))
                .block(ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL).title(title));
            f.render_widget(para, chunks_main[1]);
        }
    }
}
