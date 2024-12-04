use crate::util::read_inputs;
use anyhow::Result;
use std::collections::HashSet;
use strum::{EnumIter, IntoEnumIterator};

const GRID_SIZE_X: usize = 140;
const GRID_SIZE_Y: usize = 140;

#[derive(Debug, Copy, Clone, EnumIter)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn as_vec(&self) -> (i16, i16) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }

    fn cross() -> impl Iterator<Item = &'static Direction> {
        [
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ]
        .iter()
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
    let inputs = read_inputs(4)?;
    let mut result: i32 = 0;

    let mut grid: [[char; GRID_SIZE_X]; GRID_SIZE_Y] = [['.'; GRID_SIZE_X]; GRID_SIZE_Y];

    let mut x_positions: Vec<(usize, usize)> = Vec::new();
    let mut m_positions: Vec<(usize, usize)> = Vec::new();

    for (y, line) in inputs.iter().enumerate() {
        for (x, value) in line.char_indices() {
            grid[y][x] = value;
            if value == 'X' {
                x_positions.push((x, y));
            }
            if value == 'M' {
                m_positions.push((x, y));
            }
        }
    }

    for start_pos in x_positions {
        'dirs: for direction in Direction::iter() {
            let mut current = start_pos;
            for c in "MAS".chars() {
                let Some(new_pos) = direction.offset(current) else {
                    continue 'dirs;
                };
                current = new_pos;
                if grid[current.1][current.0] != c {
                    continue 'dirs;
                }
            }
            result += 1;
        }
    }

    println!("Day 4 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    let mut mas_centers: HashSet<(usize, usize)> = HashSet::new();
    for start_pos in m_positions {
        'dirs: for direction in Direction::cross() {
            let Some(a_pos) = direction.offset(start_pos) else {
                continue 'dirs;
            };
            let Some(s_pos) = direction.offset(a_pos) else {
                continue 'dirs;
            };

            if grid[a_pos.1][a_pos.0] != 'A' || grid[s_pos.1][s_pos.0] != 'S' {
                continue 'dirs;
            }

            if mas_centers.contains(&a_pos) {
                result += 1;
            } else {
                mas_centers.insert(a_pos);
            }
        }
    }

    println!("Day 4 Part 2: {result}");

    Ok(())
}
