use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
    for s in read_input().unwrap() {
        let c = s.chars();
        let len = s.len();
        println!("{}", len);
    }
    for pair in split_tokens() {
        println!("{}", pair);
    }


}

fn split_tokens() -> Vec<String> {
    //let range = std::ops::Range { start: 'a', end: 'z' };
    let mut ret: Vec<String> = Vec::new();

    for c in  {
        println!("{}", len);
        let mut a = c.to_string();
        let mut b = c.to_uppercase().to_string();
        a.push_str(c.to_uppercase();
        b.push_str(c.to_string());
        ret.push(a);
        ret.push(b);
    }
    return ret;
}

fn read_input() -> Result<Vec<String>> {
    let file = File::open("05/input.txt")?;
    let file = BufReader::new(file);
    let mut records: Vec<String> = Vec::new();
    for line in file.lines().filter_map(|result| result.ok()) {
        records.push(line.to_string());
    }
    return Ok(records);
}
