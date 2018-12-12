use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};


fn main() -> Result<()> {
    find_checksum()?;
    Ok(())
}

fn find_checksum() -> Result<()> {
    let file = File::open("day2/input01.txt")?;
    let file = BufReader::new(file);
    let mut twos = 0;
    let mut threes = 0;

    for line in file.lines().filter_map(|result| result.ok()) {
        let mut chars = line.chars();
        let mut char_sums = HashMap::new();

        chars.for_each(|c| *char_sums.entry(c).or_insert(0) += 1);
        if char_sums.iter().any(|(_k, v)| *v == 2) {
            twos += 1;
        }
        if char_sums.iter().any(|(_k, v)| *v == 3) {
            threes += 1;
        }
    }
    println!("{} * {} = {}", twos, threes, twos * threes);
    Ok(())
}
