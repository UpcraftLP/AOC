use crate::util::read_inputs;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::cmp::PartialEq;
use strum::{EnumIter, IntoEnumIterator};

const GRID_SIZE_X: usize = 50;
const GRID_SIZE_Y: usize = 50;

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn as_vec(&self) -> (i16, i16) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    fn offset(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        let vec = self.as_vec();
        let target_x = pos.0 as i16 + vec.0;
        let target_y = pos.1 as i16 + vec.1;

        if (0..GRID_SIZE_X as i16).contains(&target_x)
            && (0..GRID_SIZE_Y as i16).contains(&target_y)
        {
            Some((target_x as usize, target_y as usize))
        } else {
            None
        }
    }
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(10)?;
    let mut result: i32 = 0;

    let mut grid: [[u8; GRID_SIZE_X]; GRID_SIZE_Y] = [[0u8; GRID_SIZE_X]; GRID_SIZE_Y];

    for (y, line) in inputs.iter().enumerate() {
        for (x, value) in line.char_indices() {
            let height = value
                .to_digit(10)
                .with_context(|| format!("{value} is not in [0-9]"))?
                as u8;
            grid[y][x] = height;
        }
    }

    fn search<'a>(
        grid: &[[u8; GRID_SIZE_X]; GRID_SIZE_Y],
        start_pos: (usize, usize),
        start_height: u8,
        exclude: Option<Direction>,
        encountered_ends: &'a mut Vec<(usize, usize)>,
    ) -> &'a mut Vec<(usize, usize)> {
        Direction::iter().fold(encountered_ends, |acc, direction| {
            if let Some(exclude) = exclude {
                if exclude == direction {
                    return acc;
                }
            }

            if let Some(offset_pos) = direction.offset(start_pos) {
                let height = grid[offset_pos.1][offset_pos.0];
                if height == (start_height + 1) {
                    // should continue in this direction
                    if height == 9 {
                        acc.push(offset_pos);
                        return acc;
                    }

                    return search(grid, offset_pos, height, Some(direction.opposite()), acc);
                }
            }

            acc
        })
    }

    for y in 0..GRID_SIZE_Y {
        for x in 0..GRID_SIZE_X {
            let height = grid[y][x];

            if height == 0 {
                let mut ends: Vec<(usize, usize)> = Vec::new();
                search(&grid, (x, y), height, None, &mut ends);
                if !ends.is_empty() {
                    let score = ends.iter().unique().count();
                    result += score as i32;
                }
            }
        }
    }

    println!("Day 10 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    for y in 0..GRID_SIZE_Y {
        for x in 0..GRID_SIZE_X {
            let height = grid[y][x];

            if height == 0 {
                let mut ends: Vec<(usize, usize)> = Vec::new();
                search(&grid, (x, y), height, None, &mut ends);
                if !ends.is_empty() {
                    let score = ends.len();
                    result += score as i32;
                }
            }
        }
    }

    println!("Day 10 Part 2: {result}");

    Ok(())
}
