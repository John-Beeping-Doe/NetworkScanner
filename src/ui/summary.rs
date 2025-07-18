// src/ui/summary.rs

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Table, Row, Cell, Paragraph, BorderType};
use crate::AppState;
use chrono::{Local, DateTime};

fn calc_stats(history: &[crate::network::PingResult]) -> (u128, u128, u128, u128, usize, f64) {
    let mut rtts = Vec::new();
    let mut loss = 0;
    let mut jitter = 0u128;
    let mut prev: Option<u128> = None;
    for r in history {
        if let Some(rtt) = r.rtt_ms {
            if let Some(prev_rtt) = prev {
                jitter += (rtt as i128 - prev_rtt as i128).unsigned_abs();
            }
            prev = Some(rtt);
            rtts.push(rtt);
        } else {
            loss += 1;
        }
    }
    let (min, max, avg) = if !rtts.is_empty() {
        let min = *rtts.iter().min().unwrap();
        let max = *rtts.iter().max().unwrap();
        let avg = rtts.iter().sum::<u128>() / rtts.len() as u128;
        (min, max, avg)
    } else {
        (0, 0, 0)
    };
    let jitter = if rtts.len() > 1 { jitter / (rtts.len() as u128 - 1) } else { 0 };
    let loss_pct = if !history.is_empty() {
        (100 * loss) / history.len()
    } else {
        0
    };
    (min, max, avg, jitter, loss_pct, rtts.len() as f64)
}

pub fn draw_summary_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let mut chunks = vec![];
    let targets = &app.targets;
    let n = targets.len() as u16;
    let height = area.height.saturating_sub(2) / n.max(1);

    // Split area into n vertical chunks (one per target)
    for i in 0..n {
        let top = area.y + i * height;
        let h = if i + 1 == n { area.height - (height * i) } else { height };
        chunks.push(Rect { x: area.x, y: top, width: area.width, height: h });
    }

    for (idx, target) in app.targets.iter().enumerate() {
        let history = &app.ping_history[idx];
        let (min, max, avg, jitter, loss_pct, count) = calc_stats(history);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .title(Span::styled(
                format!("Target: {target} | min: {min}ms max: {max}ms avg: {avg}ms jitter: {jitter}ms loss: {loss_pct}% count: {count:.0}"),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ));

        let header_row = Row::new([
            Cell::from("Seq").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Cell::from("Time").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Cell::from("Address").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Cell::from("RTT").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Cell::from("Stat").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Cell::from("Error").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Cell::from("Timestamp").style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]);
        let rows: Vec<Row> = history.iter().rev().take(10).rev().map(|res| {
            let ts = res.timestamp.map(|t| {
                let datetime: DateTime<Local> = t.into();
                format!("{}", datetime.format("%H:%M:%S"))
            }).unwrap_or_else(|| "--:--:--".to_string());
            let timestamp = res.timestamp.map(|t| format!("{t:?}")).unwrap_or_default();
            let rtt = res.rtt_ms.map(|ms| format!("{ms}ms")).unwrap_or("-".to_string());
            let stat = if res.ok { "OK" } else { "FAIL" };
            let err = res.err.clone().unwrap_or_default();

            let style = if res.ok {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            };
            Row::new([
                Cell::from(format!("{}", res.seq)).style(style),
                Cell::from(ts).style(style),
                Cell::from(res.addr.clone()).style(style),
                Cell::from(rtt).style(style),
                Cell::from(stat).style(style),
                Cell::from(err).style(style),
                Cell::from(timestamp).style(style),
            ])
        }).collect();

        let table = Table::new(
            std::iter::once(header_row).chain(rows),
            [
                Constraint::Length(5),   // Seq
                Constraint::Length(9),   // Time
                Constraint::Length(15),  // Address
                Constraint::Length(8),   // RTT
                Constraint::Length(6),   // Stat
                Constraint::Min(15),     // Error
                Constraint::Length(24),  // Timestamp
            ]
        )
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        f.render_widget(table, chunks[idx]);
    }
}
