use anyhow::Result;

mod util;
mod day1;

fn main() -> Result<()> {
    day1::run()?;
    Ok(())
}
