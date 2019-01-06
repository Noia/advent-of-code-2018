extern crate claim;

use claim::Claim;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::string::String;

fn main() -> Result<()> {
    println!("{}", find_overlaps()?);;
    Ok(())
}

fn find_overlaps() -> Result<i32> {
    let file = File::open("03/input.txt")?;
    let file = BufReader::new(file);
    // Builds a map of string keys representing coordinates and values being the sum of inserts
    // of the same key.
    let mut claims: HashMap<String, i32> = HashMap::new();

    for line in file.lines().filter_map(|result| result.ok()) {
        let claim = Claim::new(&line);
        claim::iterate_squares(claim)
            .iter()
            .for_each(|x| *claims.entry(x.to_string()).or_insert(0) += 1);
    }

    let overlaps: i32 = claims.iter().filter(|&(_, v)| v > &1).count() as i32;
    return Ok(overlaps);
}
