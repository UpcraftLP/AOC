use anyhow::Result;
use indicatif::{MultiProgress, ParallelProgressIterator, ProgressBar, ProgressIterator};
use itertools::Itertools;
use rayon::prelude::*;

use crate::util::read_inputs;

fn check_parity(value: String, parity: &Vec<usize>) -> bool {
    let mut groups: Vec<usize> = Vec::new();

    let mut current_group: Option<usize> = None;

    value.chars().for_each(|c| {
        if c == '#' {
            current_group = Some(match current_group {
                Some(value) => value + 1,
                None => 1,
            });
        } else {
            if let Some(value) = current_group {
                groups.push(value);
            }
            current_group = None;
        }
    });
    if let Some(value) = current_group {
        groups.push(value);
    }

    return groups == *parity;
}

fn parse_lines(inputs: &Vec<String>, repeat: usize) -> Vec<(String, Vec<usize>, Vec<usize>)> {
    let mut data: Vec<(String, Vec<usize>, Vec<usize>)> = Vec::new();

    for line in inputs {
        if let Some((springs, parity)) = line.split_once(' ') {
            let unknowns: Vec<usize> = itertools::repeat_n(springs, repeat).join("?").chars().enumerate()
                .filter_map(|(idx, c)| {
                    match c {
                        '?' => Some(idx),
                        _ => None
                    }
                })
                .collect();

            data.push((itertools::repeat_n(springs, repeat).join("?"), unknowns, itertools::repeat_n(parity, repeat).join(",").split(',').filter_map(|it| it.parse::<usize>().ok()).collect()));
        }
    }

    data
}

fn parse_arrangements(data: &Vec<(String, Vec<usize>, Vec<usize>)>) -> usize {
    let pb = ProgressBar::new(data.len() as u64);
    data.iter().progress_with(pb).map(|(springs, unknowns, parity)| {
        let values = vec!['#', '.'];

        itertools::repeat_n(values, unknowns.len()).multi_cartesian_product().par_bridge().filter(|permutation| {
            let mut copy = springs.clone();

            permutation.iter().zip(unknowns.clone().into_iter()).for_each(|(ch, pos)| {
                copy.replace_range(pos..pos + 1, (*ch).to_string().as_str());
            });

            check_parity(copy, &parity)
        }).count()
    }).sum()
}

fn get_arrangements(inputs: &Vec<String>, repeat: usize) -> usize {
    let data = parse_lines(inputs, repeat);
    let arrangements = parse_arrangements(&data);

    arrangements
}

pub(crate) fn run() -> Result<()> {

    let inputs = read_inputs(12)?;
    let mut result: usize = 0;

    result = get_arrangements(&inputs, 1);

    println!("Day 12 Part 1: {result}");

    // ---------------------------------------

    result = get_arrangements(&inputs, 5);

    println!("Day 12 Part 2: {result}");

    Ok(())
}