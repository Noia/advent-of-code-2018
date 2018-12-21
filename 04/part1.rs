extern crate guards;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use guards::{RecordFragment, GuardState, GuardStateSpan, Event, DatedGuardRecord};


fn main() {
    read_input();
}

fn read_input() -> Result<Vec<RecordFragment>> {
    let file = File::open("04/input.txt")?;
    let file = BufReader::new(file);
    let mut records: Vec<RecordFragment> = Vec::new();


    // This is a horrific way to do this, but I can't seem to get past
    // the borrowing issue.
    // A Better solution would be to test for matches as we build the list of claims.
    for line in file.lines().filter_map(|result| result.ok()) {
        records.push(RecordFragment::new(&line));
    }
    records.sort();
    let mut _guard_id: Option<u32> = None;
    let mut guard_records: Vec<DatedGuardRecord> = Vec::new();
    let mut _spans: Vec<GuardStateSpan> = Vec::new();
    let mut _last_record: Option<RecordFragment> = None;

    // Walk record fragments in order to build a proper log.
    for record in records.iter() {
        match record.event {
            Event::BEGIN_SHIFT => {
                if _guard_id.is_some() {
                    // finish off previous guard record.
                    if _last_record.is_some() {
                        match _last_record.unwrap().event {
                            Event::FALL_ASLEEP => {
                                _spans.push(GuardStateSpan{
                                    state: GuardState::ASLEEP,
                                    start: 00, // TODO: Derive from last_time
                                    end: 59 // TODO: Derive from last_time
                                })
                            },
                            Event::WAKE_UP => {
                                _spans.push(GuardStateSpan{
                                    state: GuardState::AWAKE,
                                    start: 00, // TODO: Derive from last_time
                                    end: 59 // TODO: Derive from last_time
                                })
                            },
                            Event::BEGIN_SHIFT => {
                                // Guard never fell asleep.
                                _spans.push(GuardStateSpan{
                                    state: GuardState::AWAKE,
                                    start: 00,
                                    end: 59
                                })
                            }
                        }
                        guard_records.push(DatedGuardRecord{
                            date: _last_record.unwrap().date, // TODO: Derive from last_time
                            guard_id: _guard_id.unwrap(),
                            events: _spans
                        });
                    }

                }
                _guard_id = record.guard_id;
                _spans = Vec::new();
                _last_record = None
            },
            Event::FALL_ASLEEP => {
                if _last_record.is_some() {
                    if _last_record.unwrap().event == Event::WAKE_UP {
                        // start span for sleep, close previous span for awake
                        let last_time = _last_record.unwrap().time;
                        _spans.push(GuardStateSpan{
                            state: GuardState::AWAKE,
                            start: 00, // TODO: Derive from last_time
                            end: 01 // TODO: Derive from last_time
                        })
                    } else {
                        panic!("Unexpected event {} before FALL_ASLEEP", _last_record.unwrap().event);
                    }
                }
                _last_record = Some(*record);

            },
            Event::WAKE_UP => {
                if _last_record.is_some() {
                    if _last_record.unwrap().event == Event::FALL_ASLEEP {
                        // start span for awake, close span for sleep
                        _spans.push(GuardStateSpan{
                            state: GuardState::AWAKE,
                            start: 00, // TODO: Derive from last_time
                            end: 01 // TODO: Derive from last_time
                        })
                    } else {
                        panic!("Unexpected event {} before WAKE_UP", _last_record.unwrap().event);
                    }
                }
                _last_record = Some(*record);
            }
        }
    }
    return Ok(records);
}
