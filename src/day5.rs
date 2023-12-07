use std::collections::{BTreeMap, HashMap};
use anyhow::Result;
use rayon::prelude::*;
use crate::util::read_inputs;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(5)?;
    let mut result: i64;

    let map_names = vec![
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ];

    let mut seeds_opt: Option<Vec<i64>> = None;

    // name -> { src_start -> (dst_start, length) }
    let mut map_data: HashMap<&str, BTreeMap<i64, (i64, i64)>> = HashMap::new();

    let mut current_map: Option<&str> = None;
    for line in &inputs {
        if line.trim().is_empty() {
            continue;
        }

        let first_space = line.find(" ").unwrap();
        let start = &line[..first_space];

        if start.contains("-") {
            current_map = Some(start);
            continue;
        }

        let values: Vec<i64> = line.split_whitespace().filter_map(|s| s.parse::<i64>().ok()).collect();

        if start == "seeds:" {
            seeds_opt = Some(values);
            continue;
        }

        let map_name = current_map.expect("parsing error, tried to assign to non-existent map");

        let map = map_data.entry(map_name).or_insert(BTreeMap::new());

        if values.len() != 3 {
            panic!("parsing error, expected 3 values, got {}: {:?}", values.len(), values);
        }

        map.insert(values[1], (values[0], values[2]));
    }

    let map_seed = |seed: i64| -> i64 {
        let mut current = seed;

        for map_name in &map_names {
            let map = map_data.get(map_name).unwrap();

            let mapping = map.iter().rev().find(|&entry| *entry.0 <= current && *entry.0 + entry.1.1 > current
            );
            if let Some(mapping) = mapping {
                current += mapping.1.0 - mapping.0;
            }
        }
        current
    };

    let seeds = seeds_opt.expect("no seeds found");

    result = seeds.iter().map(|x| map_seed(*x)).min().expect("no location found");

    println!("Day 5 Part 1: {result}");

    // ---------------------------------------

    result = seeds.par_chunks_exact(2).flat_map(|pair| pair[0]..pair[0] + pair[1] - 1).map(|x| map_seed(x)).min().expect("no location found");
    println!("Day 5 Part 2: {result}");

    Ok(())
}