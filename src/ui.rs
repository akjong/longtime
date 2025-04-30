//! Terminal user interface rendering and event handling
//!
//! This module contains all UI-related functionality including
//! rendering the terminal interface and handling user input events.

use std::{io, str::FromStr, time::Duration};

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::App, config::TimezoneConfig};

/// Runs the application's main loop
///
/// # Arguments
///
/// * `terminal` - Terminal instance to render to
/// * `app` - Application state
///
/// # Returns
///
/// * `Result<(), io::Error>` - I/O result of the terminal operations
pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = std::time::Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.previous(),
                    KeyCode::Down => app.next(),
                    KeyCode::Right => app.adjust_time_forward(15),
                    KeyCode::Left => app.adjust_time_backward(15),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = std::time::Instant::now();
        }
    }
}

/// Renders the user interface
///
/// # Arguments
///
/// * `f` - Frame to render to
/// * `app` - Application state with timezone data
fn ui(f: &mut Frame, app: &App) {
    // Define layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Timezone list
            Constraint::Length(1), // Footer
        ])
        .split(f.area());

    render_title(f, chunks[0]);
    render_timezones(f, app, chunks[1]);
    render_footer(f, chunks[2]);
}

/// Renders the application title
///
/// # Arguments
///
/// * `f` - Frame to render to
/// * `area` - Area to render in
fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new(Text::styled(
        "LongTime - Multi-timezone Time Manager",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(title, area);
}

/// Renders the timezone list
///
/// # Arguments
///
/// * `f` - Frame to render to
/// * `app` - Application state with timezone data
/// * `area` - Area to render in
fn render_timezones(f: &mut Frame, app: &App, area: Rect) {
    // Implementation details omitted for brevity
    // This would display the timezone list with their current times and work status

    // Placeholder implementation to use parameters and avoid warnings
    let block = Block::default()
        .title(format!("Timezones ({})", app.timezone_count()))
        .borders(Borders::ALL);
    f.render_widget(block, area);
}

/// Renders the footer with keyboard shortcuts
///
/// # Arguments
///
/// * `f` - Frame to render to
/// * `area` - Area to render in
fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text = Text::from(Line::from(vec![
        Span::styled("←→", Style::default().fg(Color::Yellow)),
        Span::raw(" Adjust time | "),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(" Navigate | "),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::raw(" Quit"),
    ]));

    let footer = Paragraph::new(footer_text);
    f.render_widget(footer, area);
}

/// Determines if a given time is within work hours
///
/// # Arguments
///
/// * `now` - Current time to check
/// * `timezone_config` - Timezone configuration with work hours
///
/// # Returns
///
/// * `bool` - True if time is within work hours, false otherwise
#[allow(dead_code)]
fn is_work_hours(now: DateTime<Utc>, timezone_config: &TimezoneConfig) -> bool {
    // Parse the timezone
    if let Ok(tz) = Tz::from_str(&timezone_config.timezone) {
        let local_time = now.with_timezone(&tz);
        let naive_time = local_time.time();

        if let (Some(start), Some(end)) = (
            timezone_config.work_hours.start_time(),
            timezone_config.work_hours.end_time(),
        ) {
            return naive_time >= start && naive_time <= end;
        }
    }

    false
}
