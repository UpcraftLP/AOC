mod util;
mod day1;
mod day2;
mod day3;
mod day4;

use anyhow::Result;

fn main() -> Result<()> {
    day1::run()?;
    day2::run()?;
    day3::run()?;
    day4::run()?;
    Ok(())
}
