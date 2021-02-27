use crossterm::{
    cursor::MoveToPreviousLine,
    execute,
    style::Print,
    terminal::{Clear, ClearType},
    Result,
};
use std::io::stdout;
use std::{thread, time};
mod cli;
mod game;
mod utils;

const ONE_SECOND: time::Duration = time::Duration::from_secs(1);

fn main() -> Result<()> {
    let (grid_rows, grid_cols, time_delay, states) = cli::load_game_info();
    let game_grid = game::Grid::new(grid_rows, grid_cols);
    let mut counter = 0;

    loop {
        execute!(
            stdout(),
            Clear(ClearType::All),
            Print(game_grid.print_grid_state(counter)),
            MoveToPreviousLine(1),
        )?;
        counter += 1;
        if counter == states {
            break;
        }
        thread::sleep(ONE_SECOND * time_delay);
    }

    Ok(())
}
