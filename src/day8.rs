use std::collections::{HashMap, HashSet};

use anyhow::Result;
use num_integer::lcm;
use rayon::prelude::*;

use crate::util::read_inputs;

enum Op {
    Left,
    Right,
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(8)?;
    let mut result: i64 = 0;

    type Node = (u16, u16);

    let mut indices_size: u16 = 0;
    let mut node_indices: HashMap<&str, u16> = HashMap::new();

    let mut nodes: HashMap<u16, Node> = HashMap::new();
    let mut instructions: Option<Vec<Op>> = None;

    for line in inputs.iter().filter(|s| !s.is_empty()) {
        if let Some((name, values)) = line.split_once(" = ") {
            let idx = node_indices.entry(name).or_insert_with(|| {
                let value = indices_size;
                indices_size = indices_size + 1;
                value
            });

            let node = nodes.entry(*idx).or_default();

            node.0 = *node_indices.entry(&values[1..4]).or_insert_with(|| {
                let value = indices_size;
                indices_size = indices_size + 1;
                value
            });
            node.1 = *node_indices.entry(&values[6..9]).or_insert_with(|| {
                let value = indices_size;
                indices_size = indices_size + 1;
                value
            });
        } else {
            instructions = Some(line.chars().map(|c| match c {
                'L' => Op::Left,
                'R' => Op::Right,
                _ => panic!("Invalid op"),
            }).collect());
        }
    }
    let instructions = instructions.expect("No instructions found");

    let mut current_node_idx = node_indices.get("AAA").expect("AAA not found");
    let target_node_idx = node_indices.get("ZZZ").expect("ZZZ not found");

    let mut op_idx: usize = 0;
    while current_node_idx != target_node_idx {
        result += 1;

        if let Some((left, right)) = nodes.get(current_node_idx) {
            current_node_idx = match instructions[op_idx] {
                Op::Left => left,
                Op::Right => right,
            };
        }

        if op_idx == instructions.len() - 1 {
            op_idx = 0;
        } else {
            op_idx += 1;
        }
    }

    println!("Day 8 Part 1: {result}");

    // ---------------------------------------

    let index_to_name: HashMap<u16, &str> = node_indices.iter().map(|(&name, &idx)| (idx, name)).collect();

    let positions: Vec<u16> = nodes.keys().filter(|&idx| index_to_name.get(idx).unwrap().ends_with("A")).map(|&x| x).collect();
    let winning_positions: HashSet<u16> = nodes.keys().filter(|&idx| index_to_name.get(idx).unwrap().ends_with("Z")).map(|&x| x).collect();

    result = positions.par_iter().map(|start_pos| {
        let mut current_pos = start_pos;

        let mut op_idx = 0;
        let mut path_length: usize = 0;
        let mut initial_offset: usize = 0;
        loop {
            if winning_positions.contains(current_pos) {
                if initial_offset == 0 {
                    initial_offset = path_length;
                } else {
                    return path_length - initial_offset;
                }
            }
            path_length = path_length + 1;

            if let Some((left, right)) = nodes.get(current_pos) {
                current_pos = match instructions[op_idx] {
                    Op::Left => left,
                    Op::Right => right,
                };
            }

            if op_idx == instructions.len() - 1 {
                op_idx = 0;
            } else {
                op_idx += 1;
            }
        }
    }).reduce_with(lcm).unwrap() as i64;

    println!("Day 8 Part 2: {result}");

    Ok(())
}