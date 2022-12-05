use dotenvy::dotenv;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    getaoc::sync::execute()?;

    Ok(())
}
