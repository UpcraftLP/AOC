use crate::util::read_inputs;
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(7)?;
    let start_pos = inputs
        .first()
        .expect("invalid input")
        .find('S')
        .expect("Start point not found");

    let mut beams: HashSet<usize> = HashSet::new();
    beams.insert(start_pos);
    let result: usize = inputs[1..]
        .iter()
        .map(|line| {
            let splits = beams
                .extract_if(|pos| line.chars().nth(*pos).expect("out of bounds") == '^')
                .collect_vec();
            let len = splits.len();
            beams.extend(splits.into_iter().flat_map(|pos| vec![pos - 1, pos + 1]));

            len
        })
        .sum();

    println!("Day 7 Part 1: {result}");

    // ---------------------------------------

    let mut universes: HashMap<usize, usize> = HashMap::new();
    universes.insert(start_pos, 1);

    inputs[1..].iter().for_each(|line| {
        let splits = universes
            .extract_if(|pos, _| line.chars().nth(*pos).expect("out of bounds") == '^')
            .collect_vec();

        splits.into_iter().for_each(|(pos, value)| {
            [pos - 1, pos + 1].iter().for_each(|target_pos| {
                *universes.entry(*target_pos).or_default() += value;
            });
        });
    });

    let result: usize = universes.values().sum();

    println!("Day 7 Part 2: {result}");

    Ok(())
}
