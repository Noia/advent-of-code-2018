use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::AddAssign;

fn main() -> Result<()> {
    let file = File::open("01/input.txt")?;
    let mut frequency: i64 = 0;

    for line in BufReader::new(file)
        .lines()
        .filter_map(|result| result.ok())
    {
        let line_val: i64 = line.parse().unwrap();
        frequency.add_assign(line_val);
    }
    println!("Final frequency: {}", frequency);
    Ok(())
}
