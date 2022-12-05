use crate::date::{self, Day, Year};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// target puzzle year
    #[arg(short, long, default_value_t = date::this_year())]
    pub year: Year,

    /// target puzzle day of month
    #[arg(short, long, default_value_t = date::day_of_month())]
    pub day: Day,

    /// directory to save output files
    #[arg(default_value = "./")]
    pub outdir: PathBuf,

    /// name of file to save puzzle input
    #[arg(short, long = "input", default_value = "input")]
    pub input_filename: String,

    /// session id used for authentication.
    ///
    /// found by inspecting https://adventofcode.com/
    #[arg(short, long, env)]
    pub session: String,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
