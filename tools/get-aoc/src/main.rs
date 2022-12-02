use dotenvy::dotenv;

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    get_aoc::sync::execute()?;

    Ok(())
}
