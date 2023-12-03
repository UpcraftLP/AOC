use crate::util::read_inputs;
use anyhow::Result;

pub(crate) fn run() -> Result<()> {
    let inputs = read_inputs(1)?;
    let mut result: i32 = 0;
    for line in &inputs {
        let chars: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();

        let first = chars.first().unwrap();
        let last = chars.last().unwrap();

        let num: i32 = format!("{first}{last}").parse()?;
        result += num;
    }

    println!("Day 1 Part 1: {result}");

    // ---------------------------------------
    result = 0;

    let words = vec![
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    for line in &inputs {
        let window_size = words.iter().map(|w| w.len()).max().unwrap();

        let mut chars: Vec<char> = Vec::new();

        'window: for idx in 0..line.len() {
            let first_char = line.chars().skip(idx).next().unwrap();
            if first_char.is_ascii_digit() {
                chars.push(first_char);
                continue 'window;
            }

            let current_window_size = std::cmp::min(window_size, line.len() - idx);
            let window = &line[idx..idx + current_window_size];

            for (w_idx, word) in words.iter().enumerate().map(|(i, w)| (i, *w)) {
                if window.starts_with(word) {
                    chars.push(std::char::from_digit((w_idx + 1) as u32, 10).unwrap());
                    continue 'window;
                }
            }
        }

        let first = chars.first().unwrap();
        let last = chars.last().unwrap();

        let num: i32 = format!("{first}{last}").parse()?;
        result += num;
    }

    println!("Day 1 Part 2: {result}");

    Ok(())
}