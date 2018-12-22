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
    for line in file.lines().filter_map(|result| result.ok()) {
        records.push(RecordFragment::new(&line));
    }
    records.sort();
    return Ok(records);
}

fn parse_fragments_to_schedule() -> Vec<DatedGuardRecord> {
    let records: Vec<RecordFragment> = read_input().unwrap_or(Vec::new());

    let mut guard_records: Vec<DatedGuardRecord> = Vec::new();

    let mut _current_guard_id: Option<u32> = None;
    let mut _current_guard_spans: Vec<GuardStateSpan> = Vec::new();
    let mut _last_record: Option<RecordFragment> = None;

    // Walk record fragments in order to build a proper log.
    for record in records.iter() {
        match record.event {
            Event::BEGIN_SHIFT => {
                if _current_guard_id.is_some() {
                    // finish off previous guard record.
                    if _last_record.is_some() {
                        match _last_record.unwrap().event {
                            Event::FALL_ASLEEP => {
                                _current_guard_spans.push(GuardStateSpan{
                                    state: GuardState::ASLEEP,
                                    start: 00, // TODO: Derive from last_time
                                    end: 59 // TODO: Derive from last_time
                                })
                            },
                            Event::WAKE_UP => {
                                _current_guard_spans.push(GuardStateSpan{
                                    state: GuardState::AWAKE,
                                    start: 00, // TODO: Derive from last_time
                                    end: 59 // TODO: Derive from last_time
                                })
                            },
                            Event::BEGIN_SHIFT => {
                                // Guard never fell asleep.
                                _current_guard_spans.push(GuardStateSpan{
                                    state: GuardState::AWAKE,
                                    start: 00,
                                    end: 59
                                })
                            }
                        }
                        guard_records.push(DatedGuardRecord{
                            date: _last_record.unwrap().date, // TODO: Derive from last_time
                            guard_id: _current_guard_id.unwrap(),
                            events: _current_guard_spans
                        });
                    }

                }
                _current_guard_id = record.guard_id;
                _current_guard_spans = Vec::new();
                _last_record = None
            },
            Event::FALL_ASLEEP => {
                if _last_record.is_some() {
                    if _last_record.unwrap().event == Event::WAKE_UP {
                        // start span for sleep, close previous span for awake
                        let last_time = _last_record.unwrap().time;
                        _current_guard_spans.push(GuardStateSpan{
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
                    last_record = _last_record.unwrap();
                    if last_record.event == Event::FALL_ASLEEP {
                        // start span for awake, close span for sleep
                        _current_guard_spans.push(GuardStateSpan{
                            state: GuardState::AWAKE,
                            start: 00, // TODO: Derive from last_time
                            end: 01 // TODO: Derive from last_time
                        })
                    } else {
                        panic!("Unexpected event {} before WAKE_UP", last_record.event);
                    }
                }
                _last_record = Some(*record);
            }
        }
    }
    return guard_records;
}
