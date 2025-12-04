use crate::util::read_inputs;
use anyhow::Result;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use strum::Display;

#[derive(Debug, Display, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum Tile {
    Free,
    Blocked,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        if value == '@' {
            return Tile::Blocked;
        }

        Tile::Free
    }
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(4)?;
    let mut grid: HashMap<(i16, i16), Tile> = inputs
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .map(Tile::from)
                .enumerate()
                .map(move |(x, tile)| ((x as i16, y as i16), tile))
        })
        .collect();

    let result = grid
        .iter()
        .filter(|&(_, tile)| *tile == Tile::Blocked)
        .filter(|&(pos, _)| {
            (-1i16..=1i16)
                .flat_map(|x| (-1i16..=1i16).map(move |y| (x, y)))
                .map(|offset| (pos.0 + offset.0, pos.1 + offset.1))
                .filter_map(|lookup_pos| grid.get(&lookup_pos))
                .filter(|&elem| *elem == Tile::Blocked)
                .count()
                < 5
        })
        .count();

    println!("Day 4 Part 1: {result}");

    // ---------------------------------------

    let mut result: usize = 0;

    loop {
        let to_remove: HashSet<(i16, i16)> = grid
            .iter()
            .filter(|(_, tile)| **tile == Tile::Blocked)
            .map(|(pos, _)| *pos)
            .filter(|pos| {
                (-1i16..=1i16)
                    .flat_map(|x| (-1i16..=1i16).map(move |y| (x, y)))
                    .map(|offset| (pos.0 + offset.0, pos.1 + offset.1))
                    .filter_map(|lookup_pos| grid.get(&lookup_pos))
                    .filter(|&elem| *elem == Tile::Blocked)
                    .count()
                    < 5
            }).collect();

        if to_remove.is_empty() {
            break;
        }

        grid.retain(|pos, _| !to_remove.contains(pos));
        result += to_remove.len();
    }

    println!("Day 4 Part 2: {result}");

    Ok(())
}
