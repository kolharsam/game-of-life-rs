use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    Result,
};
use std::io::stdout;
mod cli;
mod game;
mod utils;

fn main() -> Result<()> {
    let (grid_rows, grid_cols) = cli::load_game();
    execute!(
        stdout(),
        SetForegroundColor(Color::Blue),
        SetBackgroundColor(Color::Green),
        Print(format!(
            "You have selected {} rows and {} columns",
            grid_rows, grid_cols
        )),
        ResetColor
    )?;

    Ok(())
}
