use std::io::{stdout, Write};
use clap::{App, load_yaml};
use crossterm::{
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
};

// All known options on the CLI
static ROWS: &str = "rows";
static COLUMNS: &str = "columns";

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let mut grid_rows = "50";
    let mut grid_cols = "50";
    
    if let Some(i) = matches.value_of(ROWS) {
        grid_rows = i;
        println!("Value for rows: {}", i);
    }
    if let Some(i) = matches.value_of(COLUMNS) {
        grid_cols = i;
        println!("Value for columns: {}", i);
    }
    
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Green),
        Print(format!("You have selected {} rows and {} columns", grid_rows, grid_cols)),
        ResetColor
    )?;

    Ok(())
}
