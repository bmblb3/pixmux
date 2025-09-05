#![warn(clippy::used_underscore_binding)]
use clap::Parser as _;
use color_eyre::Result;

mod app;
mod data_table;
mod image_layout;
mod ui;

use app::App;

#[derive(clap::Parser)]
#[command(about = "A tui-app for viewing images associated with tabular data")]
pub struct Args {
    #[arg(help = "Path to .csv file", value_hint = clap::ValueHint::FilePath)]
    pub file: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let terminal = ratatui::init();
    let result = App::new(args.file.into())?.run(terminal);
    ratatui::restore();
    result
}
