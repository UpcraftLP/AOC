use crate::util::read_inputs;
use anyhow::Result;
use itertools::Itertools;
use std::ops::RangeInclusive;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(5)?;

    let raw: Vec<(u64, u64)> = inputs
        .iter()
        .take_while(|line| line.contains('-'))
        .map(|line| {
            line.split('-')
                .filter_map(|s| s.parse::<u64>().ok())
                .collect_tuple()
                .expect("invalid range")
        })
        .sorted_by_key(|(start, _)| *start)
        .collect();

    let mut fresh_sorted: Vec<RangeInclusive<u64>> = Vec::new();

    let mut i = 0;
    let n = raw.len();
    while i < n {
        let &(start, mut end) = raw.get(i).expect("out of bounds");

        let mut j = i + 1;
        while j < n {
            let next = raw.get(j).expect("out of bounds");
            if next.0 > end {
                break;
            }

            end = end.max(next.1);
            j += 1;
        }

        fresh_sorted.push(start..=end);
        i = j;
    }

    let available = inputs
        .iter()
        .skip_while(|line| line.contains('-'))
        .filter_map(|s| s.parse::<u64>().ok())
        .collect_vec();

    let result = available
        .iter()
        .filter(|&x| fresh_sorted.iter().any(|range| range.contains(x)))
        .count();

    println!("Day 5 Part 1: {result}");

    // ---------------------------------------

    let result: usize = fresh_sorted.iter().map(|range| range.clone().count()).sum();

    println!("Day 5 Part 2: {result}");

    Ok(())
}
