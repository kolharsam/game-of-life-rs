use crossterm::{
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
    let (grid_rows, grid_cols) = cli::load_game();

    let new_start_set = utils::randomize_state(grid_rows, grid_cols);

    let mut number = 0;
    let mut number2 = 100;
    loop {
        execute!(
            stdout(),
            Clear(ClearType::All),
            Print(format!(
                "\t\t{}------{:?}-----{}\n",
                number, new_start_set, number2
            )),
            // MoveToPreviousLine(1)
        )?;
        number += 1;
        number2 -= 2;
        if number == 25 {
            break;
        }
        thread::sleep(ONE_SECOND);
    }

    Ok(())
}
