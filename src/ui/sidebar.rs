use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::AppState;

use chrono::{Local, DateTime};

pub fn draw_sidebar(f: &mut Frame, area: Rect, app: &AppState) {
    let mut lines = Vec::new();
    lines.push(Line::from(vec![Span::styled("Targets:", Style::default().bold())]));
    for (i, target) in app.targets.iter().enumerate() {
        let history = &app.ping_history[i];
        let rtts: Vec<_> = history.iter().filter_map(|r| r.rtt_ms).collect();
        let avg_rtt = if !rtts.is_empty() {
            rtts.iter().sum::<u128>() / rtts.len() as u128
        } else {
            0
        };
        let last_res = history.last();
        let status = if let Some(res) = last_res {
            if res.ok { "UP" } else { "DOWN" }
        } else {
            "?"
        };
        let style = if i == app.selected_target {
            Style::default().fg(Color::Yellow).bold()
        } else if status == "UP" {
            Style::default().fg(Color::Green)
        } else if status == "DOWN" {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        };

        let last_rtt = last_res.and_then(|r| r.rtt_ms).map(|rtt| format!("{rtt}ms")).unwrap_or("-".to_string());
        let loss = if !history.is_empty() {
            100 * (history.len() - rtts.len()) / history.len()
        } else {
            0
        };
        let last_ts = last_res.and_then(|r| r.timestamp).map(|ts| {
            let datetime: DateTime<Local> = ts.into();
            format!("{}", datetime.format("%H:%M:%S"))
        }).unwrap_or("--:--:--".to_string());

        lines.push(Line::from(vec![
            Span::styled(format!("> {target} [{status}] {last_rtt} avg:{avg_rtt}ms loss:{loss}% {last_ts}"), style)
        ]));
    }
    let para = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Targets"));
    f.render_widget(para, area);
}
