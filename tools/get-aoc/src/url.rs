use url::{ParseError, Url};

const BASE_URL: &str = "https://adventofcode.com/";

pub fn base_url() -> Result<Url, ParseError> {
    BASE_URL.parse::<Url>()
}

fn build_url(path: String) -> Result<Url, ParseError> {
    let base_url = base_url()?;
    base_url.join(path.as_str())
}

pub fn build_input_url(year: u16, day: u8) -> Result<Url, ParseError> {
    build_url(format!("{year}/day/{day}/input"))
}
