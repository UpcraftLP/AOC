use anyhow::Result;
use crate::util::read_inputs;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(2)?;
    let mut result = 0;

    'games: for line in &inputs {
        let mut pair = line.split(": ");
        let start = pair.next().unwrap();
        let game_id: i32 = start.split(" ").last().unwrap().parse().unwrap();

        let rest = pair.next().unwrap();
        let rounds = rest.split("; ");

        for round in rounds {
            let pairings = round.split(", ");
            for pairing in pairings {
                let mut pair = pairing.split(" ");
                let count: i32 = pair.next().unwrap().parse().unwrap();
                let color = pair.next().unwrap();

                let max = match color {
                    "red" => 12,
                    "green" => 13,
                    "blue" => 14,
                    _ => panic!("Unknown color {}", color),
                };

                if count > max {
                    continue 'games;
                }
            }
        }
        result += game_id;
    }

    println!("Day 2 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    for line in &inputs {
        let rounds = line.split(": ").last().unwrap().split("; ");

        let mut values = [1, 1, 1];

        for round in rounds {
            let pairings = round.split(", ");
            for pairing in pairings {
                let mut pair = pairing.split(" ");
                let count: i32 = pair.next().unwrap().parse().unwrap();
                let color = pair.next().unwrap();
                let idx = match color {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => panic!("Unknown color {}", color),
                };
                if values[idx] < count {
                    values[idx] = count;
                }
            }
        }
        let game: i32 = values.iter().product();
        result += game;
    }

    println!("Day 2 Part 2: {result}");

    Ok(())
}