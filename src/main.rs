use std::{error::Error, fs::read_to_string, io};

use clap::{Arg, Command};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

// Import custom modules
mod app;
mod config;
mod ui;

use app::App;
use config::Config;

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
                .help("Sets a custom config file path")
                .default_value("timezones.toml"),
        )
        .get_matches();

    // Get the config file path from the command line arguments
    let config_path = matches
        .get_one::<String>("config")
        .expect("Config argument should be present as it has a default value");

    // Read the configuration file
    let config_content = match read_to_string(config_path) {
        Ok(content) => content,
        Err(e) => {
            println!("Error: Unable to read config file '{config_path}': {e}");
            return Err(Box::new(e));
        }
    };

    // Try to parse the TOML content
    let config: Config = match toml::from_str(&config_content) {
        Ok(config) => config,
        Err(e) => {
            println!("Error: Failed to parse TOML config file: {e}");
            println!("Config file content:\n{config_content}");
            return Err(Box::new(e));
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
