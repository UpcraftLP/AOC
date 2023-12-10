use anyhow::Result;
use crate::util::read_inputs;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(6)?;
    let mut result: i32 = 1;

    let mut times = inputs.iter().find(|&x| x.starts_with("Time:")).unwrap().split_whitespace().skip(1).map(|x| x.parse::<i32>().unwrap());
    let mut distances = inputs.iter().find(|&x| x.starts_with("Distance:")).unwrap().split_whitespace().skip(1).map(|x| x.parse::<i32>().unwrap());

    while let Some(time) = times.next() {
        let current_distance_record = distances.next().expect("Distance record not found");

        let mut winning_combinations = 0;

        for speed in 1..time {
            let remaining_time = time - speed;
            let distance = remaining_time * speed;

            if distance > current_distance_record {
                winning_combinations += 1;
            }
        }

        result *= winning_combinations;
    }

    println!("Day 6 Part 1: {result}");

    // ---------------------------------------

    let time = inputs.iter().find(|&x| x.starts_with("Time:")).unwrap().split_whitespace().skip(1).fold(String::new(), |a, b| a + b).parse::<i64>().unwrap();
    let current_distance_record = inputs.iter().find(|&x| x.starts_with("Distance:")).unwrap().split_whitespace().skip(1).fold(String::new(), |a, b| a + b).parse::<i64>().unwrap();

    result = (1..time).map(|speed| {
        let remaining_time = time - speed;
        let distance = remaining_time * speed;

        distance
    }).filter(|&distance| distance > current_distance_record).count() as i32;

    println!("Day 6 Part 2: {result}");

    Ok(())
}