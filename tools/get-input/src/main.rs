use anyhow::anyhow;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let url = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("Input URL should be provided as first argument"))?;

    println!("Fetching {:?}...", url);

    let response = reqwest::get(url).await?;

    println!("Response: {:?} {}", response.version(), response.status());
    println!("Headers: {:#?}\n", response.headers());

    let body = response.text().await?;

    println!("{}", body);

    Ok(())
}
