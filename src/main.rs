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
    let mut game_grid = game::Grid::new(grid_rows, grid_cols);
    let mut counter = 0;

    loop {
        // FIXME?: the output from this may not be ideal
        // especially the clearing of screen part and the layout

        // TODO: add information like, how many are alive at that
        // particular point in time / iteration
        execute!(
            stdout(),
            Clear(ClearType::All),
            Print(game_grid.print_grid_state()),
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
