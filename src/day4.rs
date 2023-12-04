use std::collections::HashMap;
use anyhow::Result;
use crate::util::read_inputs;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(4)?;
    let mut result: i32 = 0;

    let mut cards: HashMap<i32, (Vec<i32>, Vec<i32>, i32)> = HashMap::new();

    for line in &inputs {
        let mut idx = 0;
        let mut card_id = 0;
        let mut card: (Vec<i32>, Vec<i32>, i32) = (Vec::new(), Vec::new(), 1);
        for (_, mut symbol) in line.split_whitespace().enumerate() {
            let mut is_idx = false;
            if symbol == "|" {
                idx = 2;
                continue;
            }

            if symbol.ends_with(':') {
                symbol = &symbol[..symbol.len() - 1];
                is_idx = true;
            }

            if let Some(num) = symbol.parse::<i32>().ok() {
                match idx {
                    0 => card_id = num,
                    1 => card.0.push(num),
                    2 => card.1.push(num),
                    _ => {}
                }
            }

            if is_idx {
                idx = 1;
            }
        }
        cards.insert(card_id, card);
    }

    for (_, card) in &cards {
        let count = card.1.iter().filter(|&x| card.0.contains(x)).count();
        if count > 0 {
            let score = 1 << (count - 1);
            result += score;
        }
    }

    println!("Day 4 Part 1: {result}");

    // ---------------------------------------

    for id in 1..cards.len() + 1 {
        let card = cards.get(&(id as i32)).unwrap();
        let count = card.1.iter().filter(|&x| card.0.contains(x)).count() as i32;

        let value = card.2;

        for i in 1..count + 1 {
            if let Some(target) = cards.get_mut(&(i + id as i32)) {
                target.2 += value;
            }
        }
    }

    result = cards.iter().map(|(_, card)| card.2).sum();
    println!("Day 4 Part 2: {result}");

    Ok(())
}