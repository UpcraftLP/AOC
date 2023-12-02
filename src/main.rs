mod util;
mod day1;
mod day2;

use anyhow::Result;

fn main() -> Result<()> {
    day1::run()?;
    day2::run()?;
    Ok(())
}
