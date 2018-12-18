extern crate claim;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use claim::Claim;

fn main() {
    let singleton_claim = find_unmatched();
    if singleton_claim.len() == 1 {
        println!("Found the single claim. {}", singleton_claim.get(0).unwrap().id);
        return;
    }
    if singleton_claim.len() == 0 {
        panic!("Found no singletons.");
    }

    println!("Found multiple candidates.");
    for c in singleton_claim.iter() {
        println!("{}", c);
    }
    panic!()
}

// This is a horrific way to do this, but I can't seem to get past
// the borrowing issue.
// A Better solution would be to test for matches as we build the list of claims.
fn find_unmatched() -> Vec<claim::Claim> {
    let mut diff: Vec<claim::Claim> = Vec::new();
    let mut has_match: HashSet<u32> = HashSet::new();

    let claims = read_claims().unwrap();

    for claim_a in claims.iter() {
        for claim_b in claims.iter() {
            if claim_a.id == claim_b.id {
                // Skip when they are the same ID.
                continue;
            }
            if claim_a.intersects(&claim_b) {
                has_match.insert(claim_a.id);
                has_match.insert(claim_b.id);
            }
        }
    }

    println!("{} out of {} ({}) has overlaps", has_match.len(), claims.len(), claims.len()-has_match.len());

    claims.iter()
        .filter(|&c| !has_match.contains(&c.id))
        .for_each(|c| diff.push(c.clone()));
    return diff;
}

fn read_claims() -> Result<Vec<claim::Claim>> {
    let file = File::open("03/input.txt")?;
    let file = BufReader::new(file);
    let mut claims: Vec<Claim> = Vec::new();

    // This is a horrific way to do this, but I can't seem to get past
    // the borrowing issue.
    // A Better solution would be to test for matches as we build the list of claims.
    for line in file.lines().filter_map(|result| result.ok()) {
        claims.push(Claim::new(&line));
    }
    return Ok(claims);

}
