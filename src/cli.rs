use crate::utils;
use clap::{load_yaml, App};

// All known options on the CLI
static ROWS: &str = "rows";
static COLUMNS: &str = "columns";
static DELAY: &str = "delay";

pub fn load_game() -> (u32, u32, u32) {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let mut grid_rows = "50";
    let mut grid_cols = "50";
    let mut time_delay = "1";

    if let Some(i) = matches.value_of(ROWS) {
        grid_rows = i;
    }
    if let Some(i) = matches.value_of(COLUMNS) {
        grid_cols = i;
    }
    if let Some(i) = matches.value_of(DELAY) {
        time_delay = i;
    }

    let grid_rows_num = utils::convert_to_u32(grid_rows);
    let grid_cols_num = utils::convert_to_u32(grid_cols);
    let time_delay_num = utils::convert_to_u32(time_delay);

    (grid_rows_num, grid_cols_num, time_delay_num)
}
