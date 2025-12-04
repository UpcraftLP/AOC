use crate::util::read_inputs;
use anyhow::Result;
use itertools::Itertools;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(3)?;

    let banks = inputs
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).expect("expected digit"))
                .collect_vec()
        })
        .collect_vec();

    let result: u64 = banks
        .iter()
        .map(|bank| {
            let (_, joltage) = (0..2).rev().fold((0usize, 0u64), |acc, i| {
                let max = bank
                    .iter()
                    .enumerate()
                    .rev()
                    .take(bank.len() - acc.0)
                    .skip(i)
                    .max_by_key(|(_, a)| *a)
                    .expect("no max");

                (max.0 + 1, acc.1 * 10 + *max.1 as u64)
            });

            joltage
        })
        .sum();

    println!("Day 3 Part 1: {result}");

    // ---------------------------------------

    let result: u64 = banks
        .iter()
        .map(|bank| {
            let (_, joltage) = (0..12).rev().fold((0usize, 0u64), |acc, i| {
                let max = bank
                    .iter()
                    .enumerate()
                    .rev()
                    .take(bank.len() - acc.0)
                    .skip(i)
                    .max_by_key(|(_, a)| *a)
                    .expect("no max");

                (max.0 + 1, acc.1 * 10 + *max.1 as u64)
            });

            joltage
        })
        .sum();

    println!("Day 3 Part 2: {result}");

    Ok(())
}
