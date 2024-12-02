use crate::util::read_inputs;
use anyhow::Result;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(1)?;
    let mut result: i32 = 0;

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in &inputs {
        let mut split = line.split_whitespace();
        left.push(split.next().unwrap().parse()?);
        right.push(split.next().unwrap().parse()?);
    }
    assert_eq!(left.len(), right.len());
    left.sort();
    right.sort();

    for i in 0..left.len() {
        let pair = (left[i], right[i]);

        result += pair.0.abs_diff(pair.1) as i32;
    }

    println!("Day 1 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    for &l in left.iter() {
        for &r in right.iter() {
            if l == r {
                result += l as i32;
            }
        }
    }

    println!("Day 1 Part 2: {result}");

    Ok(())
}
