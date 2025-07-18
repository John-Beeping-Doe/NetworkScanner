// src/ui/ping.rs

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Row, Table, Cell, Paragraph};
use crate::AppState;
use chrono::{Local, DateTime};

pub fn draw_ping_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let idx = 0;
    let history = &app.ping_history[idx];

    // Stats
    let mut rtts = Vec::new();
    let mut loss = 0;
    let mut jitter = 0u128;
    let mut prev: Option<u128> = None;
    for res in history {
        if let Some(rtt) = res.rtt_ms {
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
        100 * loss / history.len()
    } else {
        0
    };

    let summary = Paragraph::new(format!(
        "min: {min}ms  max: {max}ms  avg: {avg}ms  jitter: {jitter}ms  loss: {loss_pct}%"
    ))
    .block(Block::default().borders(Borders::ALL).title("Ping Results"));

    let header = Row::new([
        Cell::from("Seq").style(Style::default().bold()),
        Cell::from("Time").style(Style::default().bold()),
        Cell::from("Address").style(Style::default().bold()),
        Cell::from("RTT").style(Style::default().bold()),
        Cell::from("Stat").style(Style::default().bold()),
        Cell::from("Error").style(Style::default().bold()),
    ]);

    let rows: Vec<Row> = history.iter().rev().take(15).rev().map(|res| {
        let ts = res.timestamp.map(|t| {
            let datetime: DateTime<Local> = t.into();
            format!("{}", datetime.format("%H:%M:%S"))
        }).unwrap_or("--:--:--".to_string());
        let rtt = res.rtt_ms.map(|ms| format!("{ms}ms")).unwrap_or("-".to_string());
        let stat = if res.ok { "OK" } else { "FAIL" };
        let err = res.err.clone().unwrap_or_default();
        Row::new([
            Cell::from(format!("{}", res.seq)),
            Cell::from(ts),
            Cell::from(res.addr.clone()),
            Cell::from(rtt),
            Cell::from(stat),
            Cell::from(err),
        ])
    }).collect();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    f.render_widget(summary, chunks[0]);

    let table = Table::new(
        rows,
        [
            Constraint::Length(5),    // Seq
            Constraint::Length(9),    // Time
            Constraint::Length(13),   // Address
            Constraint::Length(8),    // RTT
            Constraint::Length(6),    // Stat
            Constraint::Min(15),      // Error
        ]
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Recent Pings"));

    f.render_widget(table, chunks[1]);
}
