//! LongTime - Multi-timezone Time Management Tool
//!
//! A terminal user interface (TUI) application for managing and displaying
//! time information across multiple time zones. Allows users to view
//! current time in different time zones, visualize work hours, and simulate
//! time changes to plan activities across time zones.
//!
//! # Features
//!
//! - Multi-timezone display with current times
//! - Work hours visualization for each time zone
//! - Time adjustment simulation
//! - Configuration via TOML file

use std::{error::Error, io};

use clap::{Arg, Command};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

// Import custom modules
mod app;
mod config;
mod ui;

use app::App;
use config::Config;

/// The main entry point for the LongTime application
///
/// Sets up the terminal interface, loads configuration,
/// and runs the application main loop.
fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments using Clap
    let matches = Command::new("longtime")
        .version("1.0")
        .about("Multi-timezone Time Manager")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file path (default: ~/.config/longtime/config.toml)"),
        )
        .get_matches();

    // Get the config file path from the command line arguments
    let config_path = matches.get_one::<String>("config").map(|s| s.as_str());

    let config = match Config::load(config_path) {
        Ok(config) => config,
        Err(e) => {
            println!("Error: Failed to load configuration: {e}");
            return Err(e);
        }
    };

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let app = App::new(config);

    // Run the main loop
    let res = ui::run_app(&mut terminal, app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}
