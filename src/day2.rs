use crate::util::read_inputs;
use anyhow::Result;
use itertools::Itertools;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(2)?;
    let inputs = inputs.first().expect("invalid input");

    let ranges = inputs
        .split_terminator(',')
        .map(|range| {
            range
                .split('-')
                .map(|e| e.parse::<u64>().expect("invalid input"))
                .collect_tuple::<(_, _)>()
                .expect("invalid range")
        })
        .collect::<Vec<_>>();

    let result: u64 = ranges
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|x| {
            let string_value = x.to_string();
            let half = string_value.len() / 2;

            string_value[half..] == string_value[..half]
        })
        .sum();

    println!("Day 2 Part 1: {result}");

    // ---------------------------------------
    let result: u64 = ranges
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .filter(|x| {
            let string_value = x.to_string();
            let half = string_value.len() / 2;

            for i in 1..=half {
                let chunks = string_value.as_bytes().chunks_exact(i);
                if !chunks.remainder().is_empty() {
                    // cannot be chunks of this length
                    continue;
                }

                let parts: Vec<_> = chunks
                    .into_iter()
                    .filter_map(|s| str::from_utf8(s).ok())
                    .collect();

                if parts.len() > 1
                    && let Some(&first) = parts.first()
                    && parts.iter().skip(1).all(|&part| part == first)
                {
                    return true;
                }
            }

            false
        })
        .sum();

    println!("Day 2 Part 2: {result}");

    Ok(())
}
