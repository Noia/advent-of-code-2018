extern crate chrono;
extern crate guards;

use chrono::Timelike;
use guards::{GuardReport, GuardState, RecordFragment};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
    let mut guards_at_bussiest_time: HashMap<u32, (u32, u32)> = HashMap::new();

    for record in read_input().unwrap().iter() {
        let mut asleep_by_hour: HashMap<u32, u32> = HashMap::new();
        record
            .records
            .iter()
            .flat_map(|r| &r.events)
            .filter(|e| e.state == GuardState::ASLEEP)
            .flat_map(|e| std::ops::Range {
                start: e.start.minute(),
                end: e.end.minute(),
            })
            .for_each(|r| *asleep_by_hour.entry(r).or_insert(0) += 1);
        let (max_asleep_hour, max_asleep_count) = asleep_by_hour
            .iter()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap()
            .clone();

        guards_at_bussiest_time.insert(record.guard_id, (*max_asleep_hour, *max_asleep_count));
    }
    let (guard_id, (minute_of_sleep, num_of_sleep)) = guards_at_bussiest_time
        .iter()
        .max_by(|(_, (_, a)), (_, (_, b))| a.cmp(b))
        .unwrap()
        .clone();
    println!(
        "#{} was asleep at 00:{} {} times. Key {}",
        guard_id,
        minute_of_sleep,
        num_of_sleep,
        (guard_id * minute_of_sleep)
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
