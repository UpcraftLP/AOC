use crate::util::read_inputs;
use anyhow::Result;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(1)?;
    let mut result: i32 = 0;

    println!("Day 1 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    println!("Day 1 Part 2: {result}");

    Ok(())
}
