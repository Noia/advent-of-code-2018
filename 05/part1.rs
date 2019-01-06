use std::fs::File;
use std::io::{BufRead, BufReader, Result};
extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(&split_tokens()).unwrap();

    let input: String = read_input().unwrap().get(0).unwrap().to_string();
    let mut collapsed_string = input.to_string();
    loop {
        let after_split: String = re.split(&collapsed_string).collect::<Vec<_>>().join("");
        if after_split.len() == collapsed_string.len() {
            collapsed_string = after_split;
            break;
        }
        collapsed_string = after_split;
    }
    println!("{} to {}", input.len(), collapsed_string.len());
}

fn split_tokens() -> String {
    //let range = std::ops::Range { start: 'a', end: 'z' };
    let mut ret: Vec<String> = Vec::new();
    let chars = "abcdefghijklmnopqrstuvwxyz".chars();
    for c in chars {
        let mut a = c.to_string();
        let mut b = c.to_uppercase().to_string();
        a.push_str(&c.to_uppercase().to_string());
        b.push_str(&c.to_string());
        ret.push(a);
        ret.push(b);
    }
    return ret.join("|");
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
