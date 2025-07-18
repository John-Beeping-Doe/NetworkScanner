// src/ui/settings.rs

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::AppState;

pub fn draw_settings_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(vec![
        Span::styled("Targets:", Style::default().bold())
    ]));

    for (i, tgt) in app.targets.iter().enumerate() {
        let mark = if app.settings_selected == i { ">" } else { " " };
        lines.push(Line::from(vec![
            Span::from(format!("{mark} {tgt}"))
        ]));
    }
    lines.push(Line::from(vec![Span::from("")])); // Spacer
    lines.push(Line::from(vec![
        Span::styled("Tests:", Style::default().bold())
    ]));

    let test_names = ["Ping", "Sweep", "DNS", "Port Scan", "Logs"];
    for (i, &name) in test_names.iter().enumerate() {
        let idx = i + app.targets.len();
        let sel = if app.settings_selected == idx { ">" } else { " " };
        let onoff = if app.tests_enabled[i] { "[ON] " } else { "[OFF]" };
        lines.push(Line::from(vec![Span::from(format!("{sel} {onoff} {name}"))]));
    }

    let para = if app.adding_target {
        Paragraph::new(format!("Enter new target: {}", app.new_target))
            .block(Block::default().borders(Borders::ALL).title("Add Target (Enter=Save, Esc=Cancel)"))
    } else {
        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Settings [a]dd [d]el [space] toggle"))
    };
    f.render_widget(para, area);
}
