mod cli;
mod date;

use std::env;

use anyhow::Context;
use cli::Cli;
use reqwest::{cookie::Jar, Client, Url};
use tokio::fs;

const BASE_URL: &str = "https://adventofcode.com/";
const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    concat!("(", env!("CARGO_PKG_REPOSITORY"), "/tools/get-input", ")"),
    " - ",
    env!("CARGO_PKG_VERSION")
);

/// Create a request client that reads `SESSION` from environment variables for auth
fn create_client(url: &Url) -> anyhow::Result<Client> {
    let session_id =
        env::var("SESSION").context("environment variable `SESSION` should be present")?;
    let cookie = format!("session={}", session_id);

    let jar = Jar::default();
    jar.add_cookie_str(cookie.as_str(), &url);

    let client = Client::builder()
        .user_agent(USER_AGENT)
        .cookie_store(true)
        .cookie_provider(jar.into())
        .build()?;

    Ok(client)
}

/// GET text data from `url`
async fn fetch_data(client: Client, url: Url) -> reqwest::Result<String> {
    println!("Fetching {:?}...", url);

    let response = client.get(url).send().await?;

    response.text().await
}

fn input_path(year: u16, day: u8) -> String {
    format!("{year}/day/{day}/input")
}

/// Main entry point to fully execute command-line program
pub async fn execute() -> anyhow::Result<()> {
    let Cli { year, day, file } = cli::parse_args();

    let base_url: Url = BASE_URL.parse()?;

    let client = create_client(&base_url)?;

    let url = base_url.join(input_path(year, day).as_str())?;

    let data = fetch_data(client, url).await?;

    println!("{}", data);

    fs::write(&file, &data)
        .await
        .with_context(|| format!("failed to write data to file at path `{}`", file.display()))?;

    Ok(())
}

/// Module with synchronous entrypoint
pub mod sync {
    #[tokio::main]
    /// Synchronous entrypoint for blocking [`execute`]
    pub async fn execute() -> anyhow::Result<()> {
        crate::execute().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn should_save_input() -> anyhow::Result<()> {
        dotenv().expect(".env file not found");

        execute().await?;

        Ok(())
    }
}
