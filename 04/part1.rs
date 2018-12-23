extern crate chrono;
extern crate guards;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use guards::{DatedGuardRecord, Event, GuardReport, GuardState, GuardStateSpan, RecordFragment};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
    let mut laziest_guard: Option<GuardReport> = None;

    for record in read_input().unwrap().iter() {
        if laziest_guard.is_none() {
            laziest_guard = Some(record.clone());
            continue;
        }
        // Computing this every time is lazy in and of itself.
        let lazy_guard_time = laziest_guard
            .as_ref()
            .unwrap()
            .minutes_in_state(GuardState::ASLEEP);
        let other_guard_time = record.minutes_in_state(GuardState::ASLEEP);
        if other_guard_time > lazy_guard_time {
            laziest_guard = Some(record.clone());
        }
    }
    if laziest_guard.is_some() {
        println!(
            "{:?} {:?} minutes asleep",
            laziest_guard,
            laziest_guard
                .as_ref()
                .unwrap()
                .minutes_in_state(GuardState::ASLEEP)
        );
    } else {
        panic!("Failed to find a lazy guard!");
    }

    let laziest_guard: GuardReport = laziest_guard.unwrap();
    let mut asleep_by_hour: HashMap<u32, u32> = HashMap::new();
    laziest_guard
        .records
        .iter()
        .flat_map(|r| &r.events)
        .filter(|e| e.state == GuardState::ASLEEP)
        .flat_map(|e| std::ops::Range {
            start: e.start.minute(),
            end: e.end.minute(),
        })
        .for_each(|r| *asleep_by_hour.entry(r).or_insert(0) += 1);
    let (sleepiest_minute, how_often_asleep) = asleep_by_hour
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();
    println!("\nMap of times{:?}", asleep_by_hour);
    println!(
        "\n#{} Found asleep {} times at 00:{} result key {}",
        laziest_guard.guard_id,
        how_often_asleep,
        sleepiest_minute,
        (sleepiest_minute * laziest_guard.guard_id)
    );
}

fn read_input() -> Result<Vec<GuardReport>> {
    let file = File::open("04/input.txt")?;
    let file = BufReader::new(file);
    let mut records: Vec<RecordFragment> = Vec::new();
    for line in file.lines().filter_map(|result| result.ok()) {
        records.push(RecordFragment::new(&line));
    }

    return Ok(guards::to_guard_reports(guards::to_dated_guard_records(
        records,
    )));
}
