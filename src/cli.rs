use crate::utils;
use clap::{load_yaml, App};

// All known options on the CLI
static ROWS: &str = "rows";
static COLUMNS: &str = "columns";

pub fn load_game() -> (u32, u32) {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let mut grid_rows = "50";
    let mut grid_cols = "50";

    if let Some(i) = matches.value_of(ROWS) {
        grid_rows = i;
    }
    if let Some(i) = matches.value_of(COLUMNS) {
        grid_cols = i;
    }

    let grid_rows_num = utils::convert_to_u32(grid_rows);
    let grid_cols_num = utils::convert_to_u32(grid_cols);

    (grid_rows_num, grid_cols_num)
}
