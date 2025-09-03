#![warn(clippy::used_underscore_binding)]
use std::{env, path};

use color_eyre::Result;

mod app;
mod data_table;
mod image_layout;
mod tab;
mod utils;

use app::App;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv_file>", args[0]);
        std::process::exit(1);
    }

    let terminal = ratatui::init();
    let result = App::new(path::PathBuf::from(&args[1]))?.run(terminal);
    ratatui::restore();
    result
}
