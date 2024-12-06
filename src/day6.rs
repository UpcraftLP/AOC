use crate::util::read_inputs;
use anyhow::Result;
use std::array;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use strum::EnumIter;

const GRID_SIZE_X: usize = 130;
const GRID_SIZE_Y: usize = 130;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Type {
    #[default]
    Free,
    Obstacle,
}

impl From<char> for Type {
    fn from(c: char) -> Self {
        match c {
            '.' => Type::Free,
            '#' => Type::Obstacle,
            _ => panic!("Invalid character: {c}"),
        }
    }
}

impl From<Type> for char {
    fn from(t: Type) -> char {
        match t {
            Type::Free => '.',
            Type::Obstacle => '#',
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", char::from(*self))
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
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

#[derive(Debug, Clone, Default)]
struct GridTile {
    tile_type: Type,
    visited: bool,
}

fn traverse(
    grid: &mut [[GridTile; GRID_SIZE_X]; GRID_SIZE_Y],
    start_pos: (usize, usize),
    override_pos: Option<(usize, usize)>,
) -> bool {
    let mut current_dir: Direction = Direction::North;
    let mut current_pos = start_pos;

    let mut visited_facing: HashMap<(usize, usize), HashSet<Direction>> = HashMap::new();

    loop {
        let Some(next_pos) = current_dir.offset(current_pos) else {
            return false;
        };
        let tile = &mut grid[next_pos.1][next_pos.0];
        let tile_type = override_pos
            .filter(|&pos| pos == next_pos)
            .map(|_| Type::Obstacle)
            .unwrap_or(tile.tile_type);
        if tile_type != Type::Free {
            current_dir = match current_dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            continue;
        }
        tile.visited = true;

        let facing = visited_facing.entry(current_pos).or_default();

        // loop detected
        if facing.contains(&current_dir) {
            return true;
        }
        facing.insert(current_dir);

        current_pos = next_pos;
    }
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(6)?;
    let mut result: i32;

    let mut grid: [[GridTile; GRID_SIZE_X]; GRID_SIZE_Y] =
        array::from_fn(|_| array::from_fn(|_| GridTile::default()));
    let mut start_pos: (usize, usize) = (0, 0);

    for (y, line) in inputs.iter().enumerate() {
        for (x, value) in line.char_indices() {
            if value == '^' {
                start_pos = (x, y);
                grid[y][x].visited = true;
            } else {
                grid[y][x].tile_type = Type::from(value);
            }
        }
    }

    if traverse(&mut grid, start_pos, None) {
        panic!("Loop detected during part 1!");
    }

    result = grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| tile.visited)
        .count() as i32;

    println!("Day 6 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    for y in 0..GRID_SIZE_Y {
        for x in 0..GRID_SIZE_X {
            let pos = (x, y);
            if grid[y][x].visited && start_pos != pos && traverse(&mut grid, start_pos, Some(pos)) {
                result += 1;
            }
        }
    }

    println!("Day 6 Part 2: {result}");

    Ok(())
}
