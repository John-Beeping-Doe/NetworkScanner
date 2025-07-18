// src/ui/ping.rs

use ratatui::{prelude::*, widgets::{Block, Borders}};
use crate::AppState;

pub fn draw_ping_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let mut lines = vec![];
    for (i, t) in app.targets.iter().enumerate() {
        let sel = if app.selected_target == i { ">" } else { " " };
        let res = app.ping_results.get(i).and_then(|r| r.as_ref());
        let line = if let Some(r) = res {
            let status = if r.ok { "OK" } else { "FAIL" };
            let rtt = r.rtt_ms.map(|v| format!("{v}ms")).unwrap_or_else(|| "N/A".to_string());
            format!("{sel} {t} – {status} – {rtt}")
        } else {
            format!("{sel} {t} – ...")
        };
        lines.push(line);
    }
    let para = ratatui::widgets::Paragraph::new(lines.join("\n"))
        .block(Block::default().borders(Borders::ALL).title("Ping Results"));
    f.render_widget(para, area);
}
