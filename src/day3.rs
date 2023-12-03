use std::collections::HashMap;
use anyhow::Result;
use crate::util::read_inputs;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(3)?;
    let mut result: i32 = 0;

    let mut part_numbers: HashMap<(usize, usize), i32> = HashMap::new();
    let mut part_symbols: HashMap<(usize, usize), char> = HashMap::new();
    let mut max_part_number_length: i32 = 0;

    for (line_number, line) in inputs.iter().enumerate() {
        let mut current_position: usize = 0;
        'line: for (idx, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                let slice = &line[current_position..idx + 1];
                let part_number = slice.parse()?;
                part_numbers.insert((line_number, current_position), part_number);

                if slice.len() as i32 > max_part_number_length {
                    max_part_number_length = slice.len() as i32;
                }

                // could be a number with more than 1 digit so don't advance the start counter
                continue 'line;
            } else if c != '.' {
                part_symbols.insert((line_number, idx), c);
            }

            current_position = idx + 1;
            continue 'line;
        }
    }

    for entry in &part_symbols {
        let pos = entry.0;

        'y: for y in pos.0 as i32 - 1..pos.0 as i32 + 2 {
            if y < 0 {
                continue 'y;
            }

            'x: for x in pos.1 as i32 - max_part_number_length..pos.1 as i32 + 2 {
                if x < 0 {
                    continue 'x;
                }

                if let Some(part_number) = part_numbers.get(&(y as usize, x as usize)) {
                    let len: i32 = part_number.to_string().len() as i32;
                    if x < pos.1 as i32 && x as i32 + len < pos.1 as i32 {
                        continue 'x;
                    }
                    result += part_number;
                }
            }
        }
    }

    println!("Day 3 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    for entry in &part_symbols {
        let pos = entry.0;
        if *entry.1 != '*' {
            // only care about gears
            continue;
        }

        let mut nums: Vec<i32> = Vec::new();

        'y: for y in pos.0 as i32 - 1..pos.0 as i32 + 2 {
            if y < 0 {
                continue 'y;
            }

            'x: for x in pos.1 as i32 - max_part_number_length..pos.1 as i32 + 2 {
                if x < 0 {
                    continue 'x;
                }

                if let Some(part_number) = part_numbers.get(&(y as usize, x as usize)) {
                    let len: i32 = part_number.to_string().len() as i32;
                    if x < pos.1 as i32 && x + len < pos.1 as i32 {
                        continue 'x;
                    }
                    nums.push(*part_number);
                }
            }
        }

        if nums.len() >= 2 {
            let ratio: i32 = nums.iter().product();
            result += ratio;
        }
    }

    println!("Day 3 Part 2: {result}");

    Ok(())
}