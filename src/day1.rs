use crate::day1::Rotation::{Left, Right};
use crate::util::read_inputs;
use anyhow::{Result, bail};
use std::str::FromStr;
use strum::Display;

#[derive(Display, Debug)]
enum Rotation {
    Right(u16),
    Left(u16),
}

impl Rotation {
    fn amount(&self) -> i32 {
        match self {
            Right(amt) => *amt as i32,
            Left(amt) => -(*amt as i32),
        }
    }
}

impl FromStr for Rotation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let Some(direction) = s.chars().nth(0) else {
            bail!("empty input")
        };
        let amount = s[1..].parse::<u16>()?;
        let result = match direction {
            'L' => Left(amount),
            'R' => Right(amount),
            x => bail!("Unknown direction: {x}"),
        };

        Ok(result)
    }
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(1)?;
    let mut result: i32 = 0;

    let mut current: i32 = 50;

    for line in inputs.iter() {
        let rot: Rotation = line.parse()?;

        current += rot.amount();
        current %= 100;

        if current == 0 {
            result += 1;
        }
    }

    println!("Day 1 Part 1: {result}");

    // ---------------------------------------
    result = 0;
    current = 50;

    for line in inputs.iter() {
        let rot: Rotation = line.parse()?;

        let amount = rot.amount();
        for i in 0..amount.abs() {
            if (current + i * amount.signum()) % 100 == 0 {
                result += 1;
            }
        }

        current += amount;
        current %= 100;

        // match rot {
        //     Left(amount) => {
        //         let (rotations, rest) = (amount as i16).div_mod_floor(&100);
        //         result += rotations as i32;
        //         current -= rest;
        //         if current < 0 {
        //             current += 100;
        //             result += 1;
        //         }
        //     }
        //     Right(amount) => {
        //         let (rotations, rest) = (current + amount as i16).div_mod_floor(&100);
        //         current = rest;
        //
        //         // if current == 0 {
        //         //     result += 1;
        //         // }
        //
        //         result += rotations as i32;
        //     }
        // }
    }

    println!("Day 1 Part 2: {result}");

    Ok(())
}
