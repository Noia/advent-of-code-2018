extern crate guards;

use guards::{DatedGuardRecord, Event, GuardState, GuardStateSpan, RecordFragment};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

fn main() {
    for record in parse_fragments_to_schedule().iter() {
        println!("{:?}", record);
    }
}

fn read_input() -> Result<Vec<RecordFragment>> {
    let file = File::open("04/input.txt")?;
    let file = BufReader::new(file);
    let mut records: Vec<RecordFragment> = Vec::new();
    for line in file.lines().filter_map(|result| result.ok()) {
        records.push(RecordFragment::new(&line));
    }
    records.sort();
    return Ok(records);
}

fn finalize_guard_record(last_record: &RecordFragment, current_guard_spans: &mut Vec<GuardStateSpan>, current_guard_id: u32) -> DatedGuardRecord {
    match last_record.event {
        Event::FALL_ASLEEP => {
            current_guard_spans.push(GuardStateSpan {
                state: GuardState::ASLEEP,
                start: 00, // TODO: Derive from last_time
                end: 59,   // TODO: Derive from last_time
            })
        }
        Event::WAKE_UP => {
            current_guard_spans.push(GuardStateSpan {
                state: GuardState::AWAKE,
                start: 00, // TODO: Derive from last_time
                end: 59,   // TODO: Derive from last_time
            })
        }
        Event::BEGIN_SHIFT => {
            // Guard never fell asleep.
            current_guard_spans.push(GuardStateSpan {
                state: GuardState::AWAKE,
                start: 00,
                end: 59,
            })
        }
    }
    return DatedGuardRecord {
        date: last_record.date.to_string(), // TODO: Derive from last_time
        guard_id: current_guard_id,
        events: current_guard_spans.to_vec(),
    };
}

fn parse_fragments_to_schedule() -> Vec<DatedGuardRecord> {
    let records: Vec<RecordFragment> = read_input().unwrap_or(Vec::new());

    let mut guard_records: Vec<DatedGuardRecord> = Vec::new();

    let mut _current_guard_id: Option<&u32> = None;
    let mut _current_guard_spans: Vec<GuardStateSpan> = Vec::new();
    let mut _last_record: Option<&RecordFragment> = None;

    // Walk record fragments in order to build a proper log.
    for record in records.iter() {
        match record.event {
            Event::BEGIN_SHIFT => {
                if _current_guard_id.is_some() {
                    // finish off previous guard record.
                    if _last_record.is_some() {
                        guard_records.push(finalize_guard_record(_last_record.as_ref().unwrap(), &mut _current_guard_spans, *_current_guard_id.unwrap()));
                    }
                }
                _current_guard_id = record.guard_id.as_ref();
                _current_guard_spans = Vec::new();
                _last_record = None
            }
            Event::FALL_ASLEEP => {
                if _last_record.is_some() {
                    if _last_record.unwrap().event == Event::WAKE_UP {
                        // start span for sleep, close previous span for awake
                        let last_time = &_last_record.unwrap().time;
                        _current_guard_spans.push(GuardStateSpan {
                            state: GuardState::AWAKE,
                            start: 00, // TODO: Derive from last_time
                            end: 01,   // TODO: Derive from last_time
                        })
                    } else {
                        panic!(
                            "Unexpected event {} before FALL_ASLEEP",
                            _last_record.unwrap().event
                        );
                    }
                }
                _last_record = Some(record);
            }
            Event::WAKE_UP => {
                if _last_record.is_some() {
                    let last_record = _last_record.unwrap();
                    if last_record.event == Event::FALL_ASLEEP {
                        // start span for awake, close span for sleep
                        _current_guard_spans.push(GuardStateSpan {
                            state: GuardState::AWAKE,
                            start: 00, // TODO: Derive from last_time
                            end: 01,   // TODO: Derive from last_time
                        })
                    } else {
                        panic!("Unexpected event {} before WAKE_UP", last_record.event);
                    }
                }
                _last_record = Some(record);
            }
        }
    }
    return guard_records;
}
