mod util;
mod day1;

use anyhow::Result;

fn main() -> Result<()> {
    day1::run()?;
    Ok(())
}
