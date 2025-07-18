// src/ui/mod.rs

use ratatui::{prelude::*, widgets::{Block, Borders}};
use crate::AppState;

mod ping;
mod sidebar;
mod settings;

pub fn draw(f: &mut Frame, app: &AppState) {
    // Layout: horizontal (sidebar | main)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(24), Constraint::Min(0)])
        .split(f.size());

    // Sidebar
    sidebar::draw_sidebar(f, chunks[0], app);

    // Main panel area: top = tabs, rest = tab content
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[1]);

    // Tab bar
    let titles: Vec<_> = crate::Tab::ALL.iter().map(|t| t.name()).collect();
    let tabs = ratatui::widgets::Tabs::new(titles)
        .select(app.current_tab)
        .block(Block::default().title("NetworkScanner Dashboard – [←/→] Tabs [q] Quit").borders(Borders::ALL))
        .highlight_style(Style::default().bold().fg(Color::Yellow));
    f.render_widget(tabs, main_chunks[0]);

    // Tab content
    match app.current_tab {
        0 => ping::draw_ping_panel(f, main_chunks[1], app),
        5 => settings::draw_settings_panel(f, main_chunks[1], app),
        n => {
            let content_title = crate::Tab::ALL[n].name();
            let para = ratatui::widgets::Paragraph::new(format!("This is the {content_title} panel."))
                .block(Block::default().borders(Borders::ALL).title(content_title));
            f.render_widget(para, main_chunks[1]);
        }
    }
}
