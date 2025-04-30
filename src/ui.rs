use std::{error::Error, io};

use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::app::App;

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    mut app: App,
) -> Result<(), Box<dyn Error>> {
    loop {
        // Draw UI
        terminal.draw(|f| ui(f, &app))?;

        // Handle events
        let timeout = app
            .tick_rate
            .checked_sub(app.last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.previous(),
                    KeyCode::Down => app.next(),
                    KeyCode::Right => app.adjust_time_forward(),
                    KeyCode::Left => app.adjust_time_backward(),
                    _ => {}
                }
            }
        }

        // Check if we need to update the time display (tick update)
        if app.last_tick.elapsed() >= app.tick_rate {
            app.last_tick = std::time::Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    // Create title
    let title = Paragraph::new("Multi-timezone Time Manager (Use Up/Down arrows to select timezone, Left/Right arrows to adjust time, Q to quit)")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Create items for the list of timezones
    let mut items: Vec<ListItem> = Vec::new();
    for tz_config in app.config.timezones.iter() {
        let time_result = app.get_current_time(&tz_config.timezone);

        if let Ok(current_time) = time_result {
            let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
            let is_working = app.is_work_hours(&current_time, &tz_config.work_hours);

            let status = if is_working {
                "Working Hours"
            } else {
                "Non-Working Hours"
            };

            let status_style = if is_working {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Red)
            };

            // Format the timezone entry with fixed column widths for alignment
            let name = format!("{:<15}", tz_config.name); // Fixed width of 15 for name
            let time = format!("{formatted_time:<20}"); // Fixed width of 20 for time
            let work_hours = format!(
                "{} - {:<10}",
                tz_config.work_hours.start, tz_config.work_hours.end
            ); // Format work hours

            let line = Span::styled(
                format!("{name} | {time} | Work Hours: {work_hours:<15} | "),
                Style::default().fg(Color::White),
            );

            let status_span = Span::styled(status, status_style);
            let combined = ratatui::text::Line::from(vec![line, status_span]);
            let list_item = ListItem::new(combined);
            items.push(list_item);
        } else {
            // Handle error case
            let error_span = Span::styled(
                format!("{:<15} | Invalid timezone", tz_config.name),
                Style::default().fg(Color::Red),
            );
            items.push(ListItem::new(error_span));
        }
    }

    // Create the timezone list with highlighting for selected item
    let timezones = List::new(items)
        .block(
            Block::default()
                .title("Timezone List")
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    // Render the timezone list with the current selection
    f.render_stateful_widget(
        timezones,
        chunks[1],
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_index)),
    );
}
