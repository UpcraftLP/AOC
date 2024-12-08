use crate::util::read_inputs;
use anyhow::Result;
use itertools::Itertools;
use num_integer::Integer;
use std::collections::{HashMap, HashSet};

const GRID_SIZE_X: usize = 50;
const GRID_SIZE_Y: usize = 50;

type Frequency = Option<char>;

fn parse_frequency(input: char) -> Frequency {
    match input {
        '.' => None,
        x => Some(x),
    }
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(8)?;
    let mut result: i32;

    let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, line) in inputs.iter().enumerate() {
        for (x, value) in line.char_indices() {
            if let Some(f) = parse_frequency(value) {
                frequencies.entry(f).or_default().push((x, y));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for positions in frequencies.values() {
        if positions.len() < 2 {
            continue;
        }

        positions.iter().permutations(2).for_each(|pair| {
            let first = pair[0];
            let second = pair[1];

            let diff = (
                second.0 as i32 - first.0 as i32,
                second.1 as i32 - first.1 as i32,
            );

            [
                (first.0 as i32 - diff.0, first.1 as i32 - diff.1),
                (second.0 as i32 + diff.0, second.1 as i32 + diff.1),
            ]
            .iter()
            .for_each(|&pos| {
                if (0..GRID_SIZE_X as i32).contains(&pos.0)
                    && (0..GRID_SIZE_Y as i32).contains(&pos.1)
                {
                    antinodes.insert(pos);
                }
            });
        });
    }

    result = antinodes.len() as i32;

    println!("Day 8 Part 1: {result}");

    // ---------------------------------------
    antinodes.clear();

    for positions in frequencies.values() {
        if positions.len() < 2 {
            continue;
        }

        positions.iter().permutations(2).for_each(|pair| {
            let first = pair[0];
            let second = pair[1];
            antinodes.insert((first.0 as i32, first.1 as i32));
            antinodes.insert((second.0 as i32, second.1 as i32));

            let diff = (
                second.0 as i32 - first.0 as i32,
                second.1 as i32 - first.1 as i32,
            );
            let gcd = diff.0.gcd(&diff.1);
            let diff = (diff.0 / gcd, diff.1 / gcd);

            let mut next_pos = (first.0 as i32 - diff.0, first.1 as i32 - diff.1);
            loop {
                if next_pos.0 < 0
                    || next_pos.0 >= GRID_SIZE_X as i32
                    || next_pos.1 < 0
                    || next_pos.1 >= GRID_SIZE_Y as i32
                {
                    break;
                }

                antinodes.insert(next_pos);
                next_pos = (next_pos.0 - diff.0, next_pos.1 - diff.1);
            }

            next_pos = (first.0 as i32 + diff.0, first.1 as i32 + diff.1);
            loop {
                if (!0..GRID_SIZE_X as i32).contains(&next_pos.0)
                    || !(0..GRID_SIZE_Y as i32).contains(&next_pos.1)
                {
                    break;
                }

                antinodes.insert(next_pos);
                next_pos = (next_pos.0 + diff.0, next_pos.1 + diff.1);
            }
        });
    }

    result = antinodes.len() as i32;

    println!("Day 8 Part 2: {result}");

    Ok(())
}
