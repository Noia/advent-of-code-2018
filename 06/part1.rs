use std::fs::File;
use std::io::{BufRead, BufReader, Result};
#[macro_use]
extern crate scan_fmt;

fn main() {
    let input = read_input().unwrap();
}

fn read_input() -> Result<Vec<(i32, i32)>> {
    let file = File::open("06/input.txt")?;
    let file = BufReader::new(file);
    let mut records: Vec<(i32, i32)> = Vec::new();
    for line in file.lines().filter_map(|result| result.ok()) {
        let (x, y) = scan_fmt!(&line, "{d}, {d}", i32, i32);
        records.push((x.unwrap(), y.unwrap()));
    }
    return Ok(records);

}
