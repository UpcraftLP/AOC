use std::fs;

use anyhow::Result;

pub fn read_inputs(day: i8) -> Result<Vec<String>> {
    let file_name = format!("day_{day:02}.txt");
    println!("Reading {}", file_name);
    let content = fs::read_to_string(format!("./input/{file_name}"))?;
    let lines = content.lines().map(String::from).collect();
    Ok(lines)
}
