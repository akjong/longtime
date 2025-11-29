//! Terminal user interface rendering and event handling
//!
//! This module contains all UI-related functionality including
//! rendering the terminal interface and handling user input events.

use std::{io, str::FromStr, time::Duration};

use chrono::{DateTime, Offset, Utc};
use chrono_tz::Tz;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    Frame, Terminal,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
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

        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
        {
            if app.is_searching {
                match key.code {
                    KeyCode::Esc | KeyCode::Enter => app.exit_search(),
                    KeyCode::Backspace => app.backspace_search(),
                    KeyCode::Char(c) => app.append_search(c),
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => app.previous(),
                    KeyCode::Down => app.next(),
                    KeyCode::Right => app.adjust_time_forward(15),
                    KeyCode::Left => app.adjust_time_backward(15),
                    KeyCode::Char('r') => app.reset_time(),
                    KeyCode::Char('?') => app.toggle_help(),
                    KeyCode::Char('/') => app.enter_search(),
                    KeyCode::Char('t') => app.toggle_format(),
                    KeyCode::Esc => {
                        if app.show_help {
                            app.toggle_help();
                        } else if !app.search_query.is_empty() {
                            app.clear_search();
                        }
                    }
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
    let constraints = if app.is_searching || !app.search_query.is_empty() {
        vec![
            Constraint::Length(3), // Title
            Constraint::Length(3), // Search
            Constraint::Min(0),    // Timezone list
            Constraint::Length(1), // Footer
        ]
    } else {
        vec![
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Timezone list
            Constraint::Length(1), // Footer
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(constraints)
        .split(f.area());

    render_title(f, chunks[0]);

    let list_area = if app.is_searching || !app.search_query.is_empty() {
        render_search(f, app, chunks[1]);
        chunks[2]
    } else {
        chunks[1]
    };

    render_timezones(f, app, list_area);

    // Footer is always the last chunk
    render_footer(f, *chunks.last().expect("Footer chunk should exist"));

    if app.show_help {
        render_help(f);
    }
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
    let header_cells = ["Name", "Time", "Diff", "Date", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Yellow)));
    let header = Row::new(header_cells)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1)
        .bottom_margin(1);

    let filtered_timezones = app.get_filtered_timezones();
    let now = app.current_time();

    // Calculate offset of the selected timezone to show relative difference
    let selected_tz_offset = if !filtered_timezones.is_empty() {
        let idx = app.selected % filtered_timezones.len();
        let (_, selected_tz_config) = filtered_timezones[idx];
        if let Ok(tz) = Tz::from_str(&selected_tz_config.timezone) {
            now.with_timezone(&tz).offset().fix().local_minus_utc()
        } else {
            0
        }
    } else {
        0
    };

    let rows = filtered_timezones
        .iter()
        .enumerate()
        .map(|(i, (_, tz_config))| {
            let (time_str, diff_str, date_str, status_str, status_style) =
                if let Ok(tz) = Tz::from_str(&tz_config.timezone) {
                    let local_time = now.with_timezone(&tz);

                    let time_format = if app.use_12h_format {
                        "%I:%M %p"
                    } else {
                        "%H:%M"
                    };
                    let time_s = local_time.format(time_format).to_string();
                    let date_s = local_time.format("%Y-%m-%d").to_string();

                    let current_offset = local_time.offset().fix().local_minus_utc();
                    let diff_seconds = current_offset - selected_tz_offset;
                    let diff_hours = diff_seconds as f64 / 3600.0;
                    let diff_s = if diff_hours == 0.0 {
                        "=".to_string()
                    } else if diff_hours > 0.0 {
                        format!("+{diff_hours}")
                    } else {
                        format!("{diff_hours}")
                    };
                    let is_working = is_work_hours(now, tz_config);
                    let (status, style) = if is_working {
                        ("WORKING", Style::default().fg(Color::Green))
                    } else {
                        ("OFF", Style::default().fg(Color::Red))
                    };
                    (time_s, diff_s, date_s, status, style)
                } else {
                    (
                        "Error".to_string(),
                        "".to_string(),
                        "".to_string(),
                        "Invalid TZ",
                        Style::default().fg(Color::Red),
                    )
                };

            let style = if i == app.selected {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };

            let cells = vec![
                Cell::from(tz_config.name.clone()),
                Cell::from(time_str),
                Cell::from(diff_str),
                Cell::from(date_str),
                Cell::from(status_str).style(status_style),
            ];
            Row::new(cells).style(style).height(1)
        });

    let t = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(20),
            Constraint::Percentage(10),
            Constraint::Percentage(25),
            Constraint::Percentage(20),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Timezones ({}) ", filtered_timezones.len())),
    );

    f.render_widget(t, area);
}

fn render_search(f: &mut Frame, app: &App, area: Rect) {
    let search_text = format!("Search: {}", app.search_query);
    let search = Paragraph::new(search_text)
        .block(Block::default().borders(Borders::ALL).title(" Filter "))
        .style(if app.is_searching {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });
    f.render_widget(search, area);
}

fn render_help(f: &mut Frame) {
    let area = centered_rect(60, 50, f.area());
    let help_text = vec![
        Line::from(Span::styled(
            "Help / Shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
            Span::raw(": Navigate list"),
        ]),
        Line::from(vec![
            Span::styled("←/→", Style::default().fg(Color::Yellow)),
            Span::raw(": Adjust time (-/+ 15m)"),
        ]),
        Line::from(vec![
            Span::styled("r", Style::default().fg(Color::Yellow)),
            Span::raw(": Reset time to now"),
        ]),
        Line::from(vec![
            Span::styled("/", Style::default().fg(Color::Yellow)),
            Span::raw(": Search/Filter timezones"),
        ]),
        Line::from(vec![
            Span::styled("t", Style::default().fg(Color::Yellow)),
            Span::raw(": Toggle 12/24h format"),
        ]),
        Line::from(vec![
            Span::styled("?", Style::default().fg(Color::Yellow)),
            Span::raw(": Toggle this help"),
        ]),
        Line::from(vec![
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(": Quit"),
        ]),
        Line::from(vec![
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Close help / Clear search"),
        ]),
    ];

    let block = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().bg(Color::DarkGray));

    f.render_widget(Clear, area);
    f.render_widget(block, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
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
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(" Reset | "),
        Span::styled("/", Style::default().fg(Color::Yellow)),
        Span::raw(" Search | "),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(" Help | "),
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

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use super::*;
    use crate::config::WorkHours;

    #[test]
    fn test_is_work_hours() {
        let tz_config = TimezoneConfig {
            name: "Test".to_string(),
            timezone: "UTC".to_string(),
            work_hours: WorkHours {
                start: "09:00".to_string(),
                end: "17:00".to_string(),
            },
        };

        // 12:00 UTC is within 09:00-17:00
        let working_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        assert!(is_work_hours(working_time, &tz_config));

        // 20:00 UTC is outside 09:00-17:00
        let off_time = Utc.with_ymd_and_hms(2023, 1, 1, 20, 0, 0).unwrap();
        assert!(!is_work_hours(off_time, &tz_config));
    }
}
