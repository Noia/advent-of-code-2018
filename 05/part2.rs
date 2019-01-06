use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
extern crate regex;
use regex::Regex;

fn main() {
    let input: String = read_input().unwrap().get(0).unwrap().to_string();

    for (k, re) in split_tokens() {
        let mut collapsed_string = input.to_string();
        loop {
            let after_split: String = re.split(&collapsed_string).collect::<Vec<_>>().join("");
            if after_split.len() == collapsed_string.len() {
                collapsed_string = after_split;
                break;
            }
            collapsed_string = after_split;
        }
        println!("{} to {}", k, collapsed_string.len());
    }
}

fn split_tokens() -> HashMap<String, Regex> {
    //let range = std::ops::Range { start: 'a', end: 'z' };
    let mut ret: Vec<String> = Vec::new();
    let chars = "abcdefghijklmnopqrstuvwxyz";
    for c in chars.chars() {
        let mut a = c.to_string();
        let mut b = c.to_uppercase().to_string();
        a.push_str(&c.to_uppercase().to_string());
        b.push_str(&c.to_string());
        ret.push(a);
        ret.push(b);
    }

    let base_pattern = ret.join("|");

    let mut map: HashMap<String, Regex> = HashMap::new();
    for c in chars.chars() {
        println!("Building pattern for {:?}", c);
        let mut pattern = c.to_string();
        pattern.push_str("|");
        pattern.push_str(&c.to_uppercase().to_string());
        pattern.push_str("|");
        pattern.push_str(&base_pattern);
        let re = Regex::new(&pattern).unwrap();
        map.insert(c.to_string(), re);
    }
    return map;
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
