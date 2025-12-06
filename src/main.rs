use anyhow::Result;
use env_logger::Env;

mod util;
//---------
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    day1::run()?;
    day2::run()?;
    day3::run()?;
    day4::run()?;
    day5::run()?;
    day6::run()?;
    Ok(())
}
