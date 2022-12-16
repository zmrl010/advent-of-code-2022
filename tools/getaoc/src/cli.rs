use crate::date::{self, Day, Year};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Target puzzle year
    #[arg(short, long, default_value_t = date::this_year())]
    pub year: Year,

    /// Target puzzle day of month
    #[arg(short, long, default_value_t = date::day_of_month())]
    pub day: Day,

    /// Directory to save output files
    #[arg(default_value = "./")]
    pub outdir: PathBuf,

    /// Name of file to save puzzle input
    #[arg(short, long = "input", default_value = "input")]
    pub input_filename: String,

    /// Session id used for authentication.
    ///
    /// This can be collected by logging into https://adventofcode.com/
    /// and inspecting the session cookie.
    #[arg(short, long, env, hide_env_values = true)]
    pub session: String,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
