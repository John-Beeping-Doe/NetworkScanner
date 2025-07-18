// src/ui/sidebar.rs

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::AppState;

pub fn draw_sidebar(f: &mut Frame, area: Rect, app: &AppState) {
    let mut lines = Vec::new();

    lines.push(Line::from(vec![
        Span::styled("Targets:", Style::default().bold()),
    ]));

    for (i, t) in app.targets.iter().enumerate() {
        let prefix = if app.selected_target == i { "> " } else { "  " };
        let style = if app.selected_target == i {
            Style::default().fg(Color::Yellow).bold()
        } else {
            Style::default()
        };
        lines.push(Line::from(vec![
            Span::styled(format!("{prefix}{t}"), style)
        ]));
    }

    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Targets"));

    f.render_widget(para, area);
}
