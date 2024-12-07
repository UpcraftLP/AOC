use crate::util::read_inputs;
use anyhow::Result;
use itertools::{repeat_n, Itertools};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Op {
    Add,
    Multiply,
    Concatenate,
}

impl Op {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Op::Add => a + b,
            Op::Multiply => a * b,
            Op::Concatenate => format!("{}{}", a, b).parse().unwrap(),
        }
    }
}

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(7)?;
    let mut result: i64 = 0;

    let mut rows: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in &inputs {
        let (expected_result, components) = line.split(": ").collect_tuple().unwrap();
        let expected_result = expected_result.parse::<i64>()?;
        let components = components
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect_vec();
        rows.push((expected_result, components));
    }

    for (expected_result, components) in &rows {
        let is_match = repeat_n(vec![Op::Add, Op::Multiply], components.len())
            .multi_cartesian_product()
            .any(|ops| {
                let actual_result = ops
                    .iter()
                    .zip(&components[1..])
                    .fold(components[0], |acc, (op, &component)| {
                        op.apply(acc, component)
                    });
                actual_result == *expected_result
            });

        if is_match {
            result += expected_result;
        }
    }

    println!("Day 7 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    for (expected_result, components) in &rows {
        let is_match = repeat_n(
            vec![Op::Add, Op::Multiply, Op::Concatenate],
            components.len(),
        )
        .multi_cartesian_product()
        .any(|ops| {
            let actual_result = ops
                .iter()
                .zip(&components[1..])
                .fold(components[0], |acc, (op, &component)| {
                    op.apply(acc, component)
                });
            actual_result == *expected_result
        });

        if is_match {
            result += expected_result;
        }
    }

    println!("Day 7 Part 2: {result}");

    Ok(())
}
