use crate::utils;
use clap::{load_yaml, App};

// All known options on the CLI
const ROWS: &str = "rows";
const COLUMNS: &str = "columns";
const DELAY: &str = "delay";
const STATES: &str = "states";

// All Default states for the cli flags
const DEFAULT_ROWS_COLUMNS: &str = "25";
const DEFAULT_TIME_DELAY: &str = "1";
const DEFAULT_STATE_ITERS: &str = "20";

pub type CLIInputs = (u32, u32, u32, u32);

pub fn load_game_info() -> CLIInputs {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let parsed_grid_rows = utils::parse_cli_flag(&matches, ROWS, DEFAULT_ROWS_COLUMNS);
    let parsed_grid_cols = utils::parse_cli_flag(&matches, COLUMNS, DEFAULT_ROWS_COLUMNS);
    let parsed_time_delay = utils::parse_cli_flag(&matches, DELAY, DEFAULT_TIME_DELAY);
    let parsed_max_states = utils::parse_cli_flag(&matches, STATES, DEFAULT_STATE_ITERS);

    (
        parsed_grid_rows,
        parsed_grid_cols,
        parsed_time_delay,
        parsed_max_states,
    )
}
