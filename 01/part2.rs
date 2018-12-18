use std::collections::HashSet;
use std::fs::File;
use std::ops::AddAssign;
use std::io::{BufRead, BufReader, Result};


fn main() -> Result<()> {
    find_stable_frequency();
    Ok(())
}

fn find_stable_frequency() -> Result<i64> {
    let mut frequency: i64 = 0;
    let mut previous_frequencies = HashSet::new();
    previous_frequencies.insert(frequency);

    loop {
        let file = File::open("01/input.txt")?;
        let file = BufReader::new(file);

        for line in file.lines().filter_map(|result| result.ok()) {
            let line_val: i64 = line.parse().unwrap();
            frequency.add_assign(line_val);
            if (!previous_frequencies.insert(frequency)) {
                println!("Final frequency: {}", frequency);
                return Ok(frequency);
            }
            println!("{}", previous_frequencies.len());
        }
    }
}
