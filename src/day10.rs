use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Formatter, Write};
use std::str::FromStr;

use anyhow::Result;

use crate::util::read_inputs;

// byte mask: NESW
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Pipe {
    Marked = 0b10000,
    Ground = 0b0000,
    Start = 0b1111,
    NorthSouth = 0b1010,
    EastWest = 0b0101,
    NorthEast = 0b1100,
    NorthWest = 0b1001,
    SouthWest = 0b0011,
    SouthEast = 0b0110,
}

impl FromStr for Pipe {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(Pipe::Start),
            "|" => Ok(Pipe::NorthSouth),
            "-" => Ok(Pipe::EastWest),
            "L" => Ok(Pipe::NorthEast),
            "J" => Ok(Pipe::NorthWest),
            "7" => Ok(Pipe::SouthWest),
            "F" => Ok(Pipe::SouthEast),
            "." => Ok(Pipe::Ground),
            "*" => Ok(Pipe::Marked),
            _ => Err(anyhow::anyhow!("Invalid pipe: {}", s)),
        }
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_char(match self {
            Pipe::Ground => '.',
            Pipe::Start => 'S',
            Pipe::Marked => '*',
            Pipe::NorthSouth => '|',
            Pipe::EastWest => '-',
            Pipe::NorthEast => 'L',
            Pipe::NorthWest => 'J',
            Pipe::SouthWest => '7',
            Pipe::SouthEast => 'F',
        })
    }
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(10)?;
    let mut result: i32;

    let mut pipe_loop: Vec<(i32, i32, Pipe)> = Vec::new();
    let mut last_pipe_pos: (i32, i32, Pipe) = (0, 0, Pipe::Start);
    let mut last_from_dir: u8 = 0b0000;

    // find start pos and first connecting piece
    for i in 0..inputs.len() {
        let mut line = &inputs[i];
        if line.trim().is_empty() {
            continue;
        }

        if let Some(start_pos) = line.find('S') {
            last_pipe_pos = (start_pos as i32, i as i32, Pipe::Start);
            pipe_loop.push(last_pipe_pos.clone());

            // find connecting piece
            let offsets: [(i32, i32, u8); 4] = [(0, 1, 0b1000), (0, -1, 0b0010), (1, 0, 0b0001), (-1, 0, 0b0100)];
            'start: for (x_off, y_off, mask) in offsets {
                let pos_x = last_pipe_pos.0 + x_off;
                let pos_y = last_pipe_pos.1 + y_off;

                // in case starting position is on the edge
                if pos_x >= line.len() as i32 || pos_x < 0
                    || pos_y >= inputs.len() as i32 || pos_y < 0 {
                    continue 'start;
                }

                line = &inputs[pos_y as usize];
                let pipe: Pipe = line.chars().nth(pos_x as usize).unwrap().to_string().parse()?;
                if (pipe as u8 & mask) == mask { // pipe connects to direction
                    last_pipe_pos = (pos_x, pos_y, pipe);
                    pipe_loop.push(last_pipe_pos.clone());
                    last_from_dir = mask;
                    break 'start;
                }
            }
        }
    }

    // traverse the loop
    'pipes: loop {
        // find next direction to go to
        let current_dir = last_pipe_pos.2 as u8 ^ last_from_dir;
        let (x_off, y_off, next_from_dir): (i32, i32, u8) = match current_dir {
            0b1000 => (0, -1, 0b0010), // go north
            0b0100 => (1, 0, 0b0001), // go east
            0b0010 => (0, 1, 0b1000), // go south
            0b0001 => (-1, 0, 0b0100), // go west
            _ => unreachable!("Invalid direction"),
        };

        let pos_x = last_pipe_pos.0 + x_off;
        let pos_y = last_pipe_pos.1 + y_off;

        let line = &inputs[pos_y as usize];
        let c = &line.chars().nth(pos_x as usize).expect("out of bounds");
        let pipe: Pipe = c.to_string().parse()?;

        if pipe == Pipe::Start {
            break 'pipes;
        }

        assert_ne!(pipe, Pipe::Ground, "No pipe here!");

        last_pipe_pos = (pos_x, pos_y, pipe);
        pipe_loop.push(last_pipe_pos.clone());
        last_from_dir = next_from_dir;
    }

    // print the loop
    inputs.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if pipe_loop.iter().find(|(x2, y2, _)| *x2 == x as i32 && *y2 == y as i32).is_some() {
                if c == 'S' {
                    print!("\x1b[0;31m{}\x1b[0m", c);
                } else {
                    print!("\x1b[0;36m{}\x1b[0m", c);
                }
            } else {
                print!("{}", c);
            }
        });
        println!();
    });
    println!();

    result = (pipe_loop.len() / 2) as i32;

    println!("Day 10 Part 1: {}", result);

    // ---------------------------------------
    result = 0;

    const GRID_SCALE: usize = 3;
    // add padding around the grid to guarantee the edges are outside the inner loop
    let size: (usize, usize) = (inputs[0].len() * GRID_SCALE + 2, inputs.len() * GRID_SCALE + 2);
    let mut grid: Vec<Vec<Pipe>> = vec![vec![Pipe::Marked; size.0]; size.1];

    let lookup: [(u8, i32, i32); 4] = [
        (0b1000, 0, -1),
        (0b0010, 0, 1),
        (0b0001, -1, 0),
        (0b0100, 1, 0),
    ];

    for (x, y, pipe) in pipe_loop {
        grid[y as usize * GRID_SCALE + 1][x as usize * GRID_SCALE + 1] = pipe;
        for (mask, x_off, y_off) in &lookup {
            if (pipe as u8 & *mask) != 0 {
                grid[(y * GRID_SCALE as i32 + y_off + 1) as usize][(x * GRID_SCALE as i32 + x_off + 1) as usize] = pipe;
            }
        }
    }

    // flood fill from top left corner
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);

    while let Some((x, y)) = queue.pop_front() {
        grid[y][x] = Pipe::Ground;

        for i in -1..2 {
            for j in -1..2 {
                let pos_x = x as i32 + i;
                let pos_y = y as i32 + j;

                if pos_x >= size.0 as i32 || pos_x < 0
                    || pos_y >= size.1 as i32 || pos_y < 0 {
                    continue;
                }

                let pos = (pos_x as usize, pos_y as usize);

                let next_pipe = grid[pos.1][pos.0];
                if next_pipe != Pipe::Marked || queue.contains(&pos) {
                    continue;
                }

                queue.push_back(pos);
            }
        }
    }

    grid.iter().enumerate().for_each(|(y, row)| {
        if y == 0 || (y - 1) % GRID_SCALE != 0 {
            return;
        }
        row.iter().enumerate().for_each(|(x, &pipe)| {
            if x == 0 || (x - 1) % GRID_SCALE != 0 {
                return;
            }

            if pipe == Pipe::Marked {
                print!("\x1b[0;33m{}\x1b[0m", pipe);
                result = result + 1;
            }
            else if pipe == Pipe::Ground {
                print!("{}", pipe);
            }
            else {
                print!("\x1b[0;37m{}\x1b[0m", pipe);
            }
        });
        println!();
    });
    println!();

    println!("Day 10 Part 2: {}", result);

    Ok(())
}