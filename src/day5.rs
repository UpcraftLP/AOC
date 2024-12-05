use crate::util::read_inputs;
use anyhow::Result;
use itertools::Itertools;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(5)?;
    let mut result: i32;

    let mut orderings: Vec<(u32, u32)> = Vec::new();
    let mut print_queues: Vec<Vec<u32>> = Vec::new();

    for line in &inputs {
        if line.contains('|') {
            orderings.push(
                line.split('|')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect_tuple::<(u32, u32)>()
                    .unwrap(),
            );
        } else if line.contains(',') {
            print_queues.push(line.split(',').map(|x| x.parse::<u32>().unwrap()).collect());
        }
    }

    let mut invalid_queues: Vec<Vec<u32>> = Vec::new();
    for &(first, second) in &orderings {
        print_queues.retain(|queue| {
            let Some(first_pos) = queue.iter().position(|&x| x == first) else {
                return true;
            };
            let Some(second_pos) = queue.iter().position(|&x| x == second) else {
                return true;
            };

            if second_pos < first_pos {
                invalid_queues.push(queue.clone());
                return false;
            }
            true
        });
    }

    result = print_queues
        .iter()
        .map(|queue| queue[queue.len() / 2] as i32)
        .sum();

    println!("Day 5 Part 1: {result}");

    // ---------------------------------------

    for queue in invalid_queues.iter_mut() {
        'sort: loop {
            for &(first, second) in &orderings {
                let Some(first_pos) = queue.iter().position(|&x| x == first) else {
                    continue;
                };
                let Some(second_pos) = queue.iter().position(|&x| x == second) else {
                    continue;
                };

                if second_pos < first_pos {
                    queue.remove(second_pos);
                    queue.insert(first_pos, second);
                    continue 'sort;
                }
            }
            break;
        }
    }

    result = invalid_queues
        .iter()
        .map(|queue| queue[queue.len() / 2] as i32)
        .sum();

    println!("Day 5 Part 2: {result}");

    Ok(())
}
