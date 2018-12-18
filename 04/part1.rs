#[macro_use] extern crate scan_fmt;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
read_input();
}

#[derive(Debug)]
pub enum Event {
    BEGIN_SHIFT,
    WAKE_UP,
    FALL_ASLEEP
}

#[derive(Debug)]
pub struct Record {
    pub date_time: String,
    pub guard_id: Option<u32>,
    pub event: Event
}

fn read_input() -> Result<Vec<Record>> {
    let file = File::open("04/input.txt")?;
    let file = BufReader::new(file);
    let mut claims: Vec<Record> = Vec::new();


    // This is a horrific way to do this, but I can't seem to get past
    // the borrowing issue.
    // A Better solution would be to test for matches as we build the list of claims.
    for line in file.lines().filter_map(|result| result.ok()) {

        // Hour is always going to be 00, but we track it anyway for correctness.
        let (date_time, record_value) = scan_fmt!(
            &line,
            // [1518-03-19 00:00]
            "[{/\\d+-\\d+-\\d+ \\d+:\\d+/}] {/.*/}",
            String, String
        );

        let record_raw = record_value.unwrap();
        let record: Record;
        match record_raw.as_ref() {
            "falls asleep" => {
                record = Record{
                        date_time: date_time.unwrap(),
                        guard_id: None,
                        event: Event::FALL_ASLEEP
                };
            },
            "wakes up" => { record = Record{
                    date_time: date_time.unwrap(),
                    guard_id: Option::None,
                    event: Event::WAKE_UP
            }; },
            _ => {
                let raw_guard_id = scan_fmt!(
                    &record_raw,
                    "Guard #{d} begins shift",
                    u32
                );
                record = Record{
                        date_time: date_time.unwrap(),
                        guard_id: Some(raw_guard_id.unwrap()),
                        event: Event::BEGIN_SHIFT
                };
            }
        }


        println!("{:?} {:?}", record_raw, record);
    }
    return Ok(claims);

}
