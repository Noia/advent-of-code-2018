use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::io::{Error, ErrorKind};
use std::string::String;
use std::iter::FromIterator;


fn main() -> Result<()> {
    println!("{}", find_checksum()?);;
    Ok(())
}

fn find_checksum() -> Result<String> {
    let file = File::open("day2/input01.txt")?;
    let file = BufReader::new(file);
    let mut prev: HashSet<String> = HashSet::new();

    for line in file.lines().filter_map(|result| result.ok()) {
        for prev_line in prev.iter() {
            let mismatches = prev_line.chars().zip(line.chars()).filter(|(a,b)| a != b).count();

            if mismatches == 1 {
                let matched_result = prev_line.chars().zip(line.chars()).filter(|(a,b)| a == b).map(|(_,b) | b);
                let matched_result = String::from_iter(matched_result);

                return Ok(matched_result);
            }
        }
        prev.insert(line);
    }
    Err(Error::new(ErrorKind::Other, "Failed to find match!"))
}
