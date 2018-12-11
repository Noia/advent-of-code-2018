use std::fs::File;
use std::ops::AddAssign;
use std::io::{BufRead, BufReader, Result};

fn main() -> Result<()> {
    let file = File::open("day1/input01.txt")?;
    let mut frequency: i64 = 0;

    for line in BufReader::new(file).lines().filter_map(|result| result.ok()) {
        let line_val: i64 = line.parse().unwrap();
        frequency.add_assign(line_val);
    }
    println!("Final frequency: {}", frequency);
    Ok(())
}
