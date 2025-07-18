// src/ui/settings.rs

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Table, Row, Cell};
use crate::AppState;

const SERVICE_NAMES: [&str; 5] = ["Ping", "Sweep", "DNS", "PortScan", "Logs"];

pub fn draw_settings_panel(f: &mut Frame, area: Rect, app: &AppState) {
    let mut rows = Vec::new();
    // Header
    let mut header_cells = vec![
        Cell::from("Idx").style(Style::default().bold()),
        Cell::from("Target").style(Style::default().bold()),
        Cell::from("Remove?").style(Style::default().bold()),
    ];
    header_cells.extend(SERVICE_NAMES.iter().map(|n| Cell::from(*n).style(Style::default().bold())));
    rows.push(Row::new(header_cells));

    for (i, t) in app.targets.iter().enumerate() {
        let mut cells = vec![
            Cell::from(format!("{}", i + 1)),
            Cell::from(t.clone()),
            Cell::from("[d]"),
        ];
        for j in 0..SERVICE_NAMES.len() {
            let selected = app.settings_row == i && app.settings_col == j;
            let mark = if app.tests_enabled[i][j] { "[X]" } else { "[ ]" };
            let cell = if selected {
                Cell::from(mark).style(Style::default().fg(Color::Yellow).add_modifier(Modifier::REVERSED | Modifier::BOLD))
            } else {
                Cell::from(mark)
            };
            cells.push(cell);
        }
        rows.push(Row::new(cells));
    }
    // Instructions row
    rows.push(Row::new(vec![
        Cell::from("Tab: Switch Tab   ↑↓→←: Move   Space/Enter: Toggle   q: Quit")
            .style(Style::default().fg(Color::Blue).bold())
    ]));

    let mut constraints = vec![
        Constraint::Length(5),
        Constraint::Length(16),
        Constraint::Length(8),
    ];
    constraints.extend(std::iter::repeat(Constraint::Length(9)).take(SERVICE_NAMES.len()));

    let table = Table::new(
        rows,
        constraints
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title("Targets [Tab] Switch Tab   [Space/Enter] Toggle   [q] Quit")
    );

    f.render_widget(table, area);
}
