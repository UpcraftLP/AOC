use crate::util::read_inputs;
use anyhow::Result;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(2)?;
    let mut result_1: i32 = 0;
    let mut result_2: i32 = 0;

    fn is_valid_sequence(seq: &[u32]) -> bool {
        let dir = seq[1] > seq[0];

        let mut cur = seq[0];
        for &next in seq.iter().skip(1) {
            let diff = next.abs_diff(cur);

            if !(1..=3).contains(&diff) || ((next > cur) != dir) {
                return false;
            }

            cur = next;
        }

        true
    }

    for line in &inputs {
        let current_report = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        // part 1:
        if is_valid_sequence(&current_report) {
            result_1 += 1;
            result_2 += 1;
        } else {
            // part 2:
            'outer: for n in 0..current_report.len() {
                let nums = current_report
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != n)
                    .map(|(_, &x)| x)
                    .collect::<Vec<u32>>();

                if is_valid_sequence(&nums) {
                    result_2 += 1;
                    break 'outer;
                }
            }
        }
    }

    println!("Day 1 Part 1: {result_1}");

    // ---------------------------------------

    println!("Day 1 Part 2: {result_2}");

    Ok(())
}
