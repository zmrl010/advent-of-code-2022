use super::date;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// target puzzle year
    #[arg(short, long, default_value_t = date::this_year())]
    pub year: u16,

    /// target puzzle day of month
    #[arg(short, long, default_value_t = date::day_of_month())]
    pub day: u8,

    /// path of file to save input
    #[arg(default_value = "./input")]
    pub file: PathBuf,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
