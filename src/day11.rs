use crate::util::read_inputs;
use anyhow::Result;
use itertools::Itertools;

use rayon::prelude::*;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(11)?;
    let mut result: usize;


    // step 1: parse image
    let galaxies: Vec<(usize, usize)> = inputs.par_iter().enumerate().flat_map(|(y, line)| {
        line.chars().enumerate().filter(|(_, c)| *c == '#').map(|(x, _)| (x, y)).collect::<Vec<(usize, usize)>>()
    }).collect();

    let mut galaxies_1 = galaxies.clone();

    // step 2: expand distances
    let mut factor: usize = 2;

    // columns
    let mut x: usize = 0;
    while galaxies_1.iter().max_by_key(|(gx, _)| *gx).unwrap().0 > x {
        if galaxies_1.iter().find(|(gx, _)| *gx == x).is_none() {
            // expand all columns after this one
            galaxies_1 = galaxies_1.iter().map(|&it| {
                if it.0 > x {
                    (it.0 + factor - 1, it.1)
                } else {
                    it
                }
            }).collect();
            x = x + factor - 1;
        }
        x = x + 1;
    }

    //rows
    let mut y: usize = 0;
    while galaxies_1.iter().max_by_key(|(_, gy)| *gy).unwrap().1 > y {
        if galaxies_1.iter().find(|(_, gy)| *gy == y).is_none() {
            // expand all rows after this one
            galaxies_1 = galaxies_1.iter().map(|&it| {
                if it.1 > y {
                    (it.0, it.1 + factor - 1)
                } else {
                    it
                }
            }).collect();
            y = y + factor - 1;
        }
        y = y + 1;
    }

    // step 3: calculate distances
    result = galaxies_1.iter().tuple_combinations().map(|(g1, g2)| {
        let dx = g1.0.abs_diff(g2.0);
        let dy = g1.1.abs_diff(g2.1);
        dx + dy
    }).sum::<usize>();

    println!("Day 11 Part 1: {result}");

    // ---------------------------------------
    galaxies_1 = galaxies.clone();

    // repeat step 2: expand distances
    factor = 1_000_000;

    // columns
    x = 0;
    while galaxies_1.iter().max_by_key(|(gx, _)| *gx).unwrap().0 > x {
        if galaxies_1.iter().find(|(gx, _)| *gx == x).is_none() {
            // expand all columns after this one
            galaxies_1 = galaxies_1.iter().map(|&it| {
                if it.0 > x {
                    (it.0 + factor - 1, it.1)
                } else {
                    it
                }
            }).collect();
            x = x + factor - 1;
        }
        x = x + 1;
    }

    //rows
    y = 0;
    while galaxies_1.iter().max_by_key(|(_, gy)| *gy).unwrap().1 > y {
        if galaxies_1.iter().find(|(_, gy)| *gy == y).is_none() {
            // expand all rows after this one
            galaxies_1 = galaxies_1.iter().map(|&it| {
                if it.1 > y {
                    (it.0, it.1 + factor - 1)
                } else {
                    it
                }
            }).collect();
            y = y + factor - 1;
        }
        y = y + 1;
    }

    // step 3: calculate distances
    result = galaxies_1.iter().tuple_combinations().map(|(g1, g2)| {
        let dx = g1.0.abs_diff(g2.0);
        let dy = g1.1.abs_diff(g2.1);
        dx + dy
    }).sum::<usize>();

    println!("Day 11 Part 2: {result}");

    Ok(())
}