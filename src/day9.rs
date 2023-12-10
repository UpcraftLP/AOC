use anyhow::Result;
use rayon::prelude::*;

use crate::util::read_inputs;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(9)?;
    let mut result: i32;

    let sequences: Vec<Vec<Vec<i32>>> = inputs.par_iter().filter(|&l| !l.trim().is_empty()).map(|line| {
        let numbers: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        let mut all: Vec<Vec<i32>> = Vec::new();
        all.push(numbers);
        let mut row_idx = 0;
        loop {
            let row = all.get(row_idx).unwrap();
            assert!(row.len() > 1, "parsing error, expected at least 2 values, got {:?} for input {:?}", row, line);

            if row.iter().all(|&x| x == 0) {
                break;
            }
            let mut next_row: Vec<i32> = Vec::new();
            for i in 1..row.len() {
                next_row.push(row[i] - row[i - 1]);
            }
            all.push(next_row);
            row_idx += 1;
        }
        all
    }).collect();

    result = sequences.par_iter().map(|seq| {
        let mut additions: Vec<i32> = vec![0];
        for row in seq.iter().rev() {
            let last_value = additions.last().unwrap();
            let current = row.last().unwrap();
            additions.push(*current + *last_value);
        }

        *additions.last().unwrap()
    }).sum::<i32>();

    println!("Day 9 Part 1: {result}");

    // ---------------------------------------

    result = sequences.par_iter().map(|seq| {
        let mut additions: Vec<i32> = Vec::new();
        additions.push(0);
        for row in seq.iter().rev() {
            let last_value = additions.last().unwrap();
            let current = row.first().unwrap();
            additions.push(*current - *last_value);
        }

        *additions.last().unwrap()
    }).sum::<i32>();

    println!("Day 9 Part 2: {result}");

    Ok(())
}