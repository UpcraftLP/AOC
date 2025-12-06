use crate::util::read_inputs;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;

enum Operation {
    Add,
    Multiply,
}

impl TryFrom<char> for Operation {
    type Error = String;

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Multiply),
            _ => Err(format!("Unknown operation: {c}")),
        }
    }
}

impl Operation {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operation::Add => a + b,
            Operation::Multiply => a * b,
        }
    }
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(6)?;

    let nums: HashMap<usize, Vec<u64>> = inputs
        .iter()
        .flat_map(|line| line.split_ascii_whitespace().enumerate())
        .filter_map(|(idx, value)| value.parse::<u64>().ok().map(|val| (idx, val)))
        .fold(HashMap::new(), |mut map, (idx, value)| {
            map.entry(idx).or_default().push(value);
            map
        });

    let mut ops = inputs
        .iter()
        .flat_map(|line| {
            line.char_indices()
                .filter(|(_, c)| !c.is_ascii_whitespace())
        })
        .filter_map(|(idx, value)| value.try_into().ok().map(|op: Operation| (idx, op)))
        .collect_vec();

    let result: u64 = ops
        .iter()
        .zip(nums.iter().sorted().map(|(_, value)| value))
        .map(|((_, op), values)| {
            values
                .iter()
                .copied()
                .reduce(|a, b| op.apply(a, b))
                .expect("empty iterator")
        })
        .sum();

    println!("Day 6 Part 1: {result}");

    // ---------------------------------------

    ops.push((usize::MAX, Operation::Add));

    let rotated = inputs
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.char_indices().rev().map(move |(x, c)| ((x, y), c)))
        .sorted_by_key(|((y, _), _)| *y)
        .fold(
            HashMap::new(),
            |mut map: HashMap<usize, String>, ((x, _), c)| {
                if c != '+' && c != '*' {
                    map.entry(x).or_default().push(c);
                }

                map
            },
        )
        .iter()
        .sorted()
        .map(|(_, line)| line.trim().to_string())
        .collect_vec();

    let cols: HashMap<usize, u64> = rotated
        .iter()
        .enumerate()
        .filter_map(|(idx, value)| value.parse::<u64>().ok().map(|value| (idx, value)))
        .collect();

    let result: u64 = ops
        .iter()
        .tuple_windows()
        .filter_map(|(&(min_idx, ref op), &(max_idx, _))| {
            cols.iter()
                .filter(|(idx, _)| **idx >= min_idx && **idx < max_idx)
                .map(|(_, &value)| value)
                .reduce(|a, b| op.apply(a, b))
        })
        .sum();

    println!("Day 6 Part 2: {result}");

    Ok(())
}
