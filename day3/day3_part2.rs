extern crate claim;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use claim::Claim;

fn main() -> Result<()> {
    println!("{:?}", find_unmatched()?);;
    Ok(())
}

fn find_unmatched() -> Result<HashSet<&'static claim::Claim>> {
    let file = File::open("day3/input.txt")?;
    let file = BufReader::new(file);
    let diff;
    let mut claims: HashSet<claim::Claim> = HashSet::new();
    let mut has_match: HashSet<u32> = HashSet::new();

    // This is a horrific way to do this, but I can't seem to get past
    // the borrowing issue.
    // A Better solution would be to test for matches as we build the list of claims.
    for line in file.lines().filter_map(|result| result.ok()) {
        let claim = Claim::new(&line);
        claims.insert(claim);
    }

    for claim_a in claims.iter() {
        if has_match.contains(&claim_a.id) {
            // Already tested
            continue;
        }
        for claim_b in claims.iter() {
            if claim_a == claim_b {
                // Skip when they are the same.
                continue;
            }
            if has_match.contains(&claim_b.id) {
                // Already tested
                continue;
            }
            if claim::intersects(&claim_a, &claim_b) {
                has_match.insert(claim_a.id);
                has_match.insert(claim_b.id);
                break;
            }
        }
    }

    diff = claims.iter().filter(|c| !has_match.contains(&c.id)).collect();

    return Ok(diff)
}
