mod cli;
mod date;
mod url;

use std::{env, path::Path};

use crate::{cli::Cli, url::base_url};
use anyhow::Context;
use reqwest::{cookie::Jar, Client};
use tokio::fs;

const USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " (",
    env!("CARGO_PKG_REPOSITORY"),
    ") - ",
    env!("CARGO_PKG_VERSION")
);

/// Create a request client
fn create_client(session: String) -> anyhow::Result<Client> {
    let base_url = base_url()?;
    let cookie = format!("session={session}");

    let jar = Jar::default();
    jar.add_cookie_str(cookie.as_str(), &base_url);

    let client = Client::builder()
        .user_agent(USER_AGENT)
        .cookie_store(true)
        .cookie_provider(jar.into())
        .build()?;

    Ok(client)
}

/// Main entry point to fully execute command-line program
pub async fn execute() -> anyhow::Result<()> {
    let Cli {
        year,
        day,
        outdir,
        input_filename,
        session,
    } = cli::parse_args();

    let client = create_client(session)?;

    let input_url = url::build_input_url(year, day)?;

    let input_data = client.get(input_url).send().await?.text().await?;
    println!("{}", input_data);

    write_file(&outdir.join(input_filename), input_data).await?;

    Ok(())
}

async fn write_file<P: AsRef<Path>, D: AsRef<[u8]>>(path: &P, data: D) -> anyhow::Result<()> {
    let result = fs::write(path, &data).await.with_context(|| {
        format!(
            "failed to write to file at path `{}`",
            path.as_ref().display()
        )
    })?;
    Ok(result)
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
