use anyhow::Result;

mod day1;
mod day2;
mod day3;
mod util;

fn main() -> Result<()> {
    day1::run()?;
    day2::run()?;
    day3::run()?;
    Ok(())
}
