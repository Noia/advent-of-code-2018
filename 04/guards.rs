#[macro_use]
extern crate scan_fmt;
extern crate chrono;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime, Timelike};
use std::cmp;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{HashMap, LinkedList};
use std::fmt;
use std::fmt::Display;
use std::num::Wrapping;
use std::string::String;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Event {
    BEGIN_SHIFT,
    WAKE_UP,
    FALL_ASLEEP,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GuardState {
    AWAKE,
    ASLEEP,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RecordFragment {
    pub date_time: NaiveDateTime,
    pub guard_id: Option<u32>,
    pub event: Event,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct GuardStateSpan {
    pub state: GuardState,
    pub start: NaiveTime,
    pub end: NaiveTime,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DatedGuardRecord {
    pub date: NaiveDate,
    pub guard_id: u32,
    pub events: Vec<GuardStateSpan>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GuardReport {
    pub guard_id: u32,
    pub records: Vec<DatedGuardRecord>,
}

impl Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialOrd for RecordFragment {
    fn partial_cmp(&self, other: &RecordFragment) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RecordFragment {
    fn cmp(&self, other: &RecordFragment) -> Ordering {
        self.date_time.cmp(&other.date_time)
    }
}

impl GuardReport {
    pub fn minutes_in_state(&self, state: GuardState) -> u32 {
        let sum = self
            .records
            .iter()
            .fold(0, |sum, r| sum + r.minutes_in_state(state));
        return sum;
    }
}

impl DatedGuardRecord {
    pub fn minutes_in_state(&self, state: GuardState) -> u32 {
        let sum = self
            .events
            .iter()
            .filter(|s| s.state == state)
            .fold(0, |sum, s| sum + (s.end.minute() - s.start.minute()));
        return sum;
    }
}

impl RecordFragment {
    pub fn new(from: &str) -> RecordFragment {
        let (y, m, d, h, mn, record_value) = scan_fmt!(
            from,
            // [1518-03-19 00:00]
            // [1518-04-12 00:36] falls asleep
            "[{d}-{d}-{d} {d}:{d}] {/.*/}",
            i32,
            u32,
            u32,
            u32,
            u32,
            String
        );
        if record_value.is_none() {
            panic!(
                "Invalid format \"{}\" {:?}-{:?}-{:?} {:?}:{:?} ...",
                from, y, m, d, h, mn
            );
        }
        let date_time: NaiveDateTime = NaiveDate::from_ymd(y.unwrap(), m.unwrap(), d.unwrap())
            .and_hms(h.unwrap(), mn.unwrap(), 00);
        let record_raw = record_value.unwrap();
        let record: RecordFragment;
        match record_raw.as_ref() {
            "falls asleep" => {
                record = RecordFragment {
                    date_time: date_time,
                    guard_id: None,
                    event: Event::FALL_ASLEEP,
                };
            }
            "wakes up" => {
                record = RecordFragment {
                    date_time: date_time,
                    guard_id: Option::None,
                    event: Event::WAKE_UP,
                };
            }
            _ => {
                let raw_guard_id = scan_fmt!(&record_raw, "Guard #{d} begins shift", u32);
                record = RecordFragment {
                    date_time: date_time,
                    guard_id: Some(raw_guard_id.unwrap()),
                    event: Event::BEGIN_SHIFT,
                };
            }
        }
        return record;
    }
}

pub fn to_dated_guard_records(records: Vec<RecordFragment>) -> Vec<DatedGuardRecord> {
    let mut records: Vec<RecordFragment> = records.clone();
    records.sort();
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
                        guard_records.push(finalize_guard_record(
                            _last_record.as_ref().unwrap(),
                            &mut _current_guard_spans,
                            *_current_guard_id.unwrap(),
                        ));
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
                        let last_time = &_last_record.unwrap().date_time.time();
                        _current_guard_spans.push(GuardStateSpan {
                            state: GuardState::AWAKE,
                            start: *last_time,
                            end: record.date_time.time(),
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
                        let last_time = &_last_record.unwrap().date_time.time();
                        _current_guard_spans.push(GuardStateSpan {
                            state: GuardState::ASLEEP,
                            start: *last_time,
                            end: record.date_time.time(),
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

fn finalize_guard_record(
    last_record: &RecordFragment,
    current_guard_spans: &mut Vec<GuardStateSpan>,
    current_guard_id: u32,
) -> DatedGuardRecord {
    let last_time = last_record.date_time.time();
    match last_record.event {
        Event::FALL_ASLEEP => {
            current_guard_spans.push(GuardStateSpan {
                state: GuardState::ASLEEP,
                start: last_time,
                end: NaiveTime::from_hms(last_time.hour(), 59, 59), // TODO: Derive from last_time
            })
        }
        Event::WAKE_UP => {
            current_guard_spans.push(GuardStateSpan {
                state: GuardState::AWAKE,
                start: last_time, // TODO: Derive from last_time
                end: NaiveTime::from_hms(last_time.hour(), 59, 59), // TODO: Derive from last_time
            })
        }
        Event::BEGIN_SHIFT => {
            // Guard never fell asleep.
            current_guard_spans.push(GuardStateSpan {
                state: GuardState::AWAKE,
                start: NaiveTime::from_hms(00, 00, 00),
                end: NaiveTime::from_hms(00, 59, 59),
            })
        }
    }
    return DatedGuardRecord {
        date: last_record.date_time.date(), // TODO: Derive from last_time
        guard_id: current_guard_id,
        events: current_guard_spans.to_vec(),
    };
}

pub fn to_guard_reports(records: Vec<DatedGuardRecord>) -> Vec<GuardReport> {
    let mut mapped_records: HashMap<u32, Vec<DatedGuardRecord>> = HashMap::new();
    for record in records.iter() {
        if !mapped_records.contains_key(&record.guard_id) {
            mapped_records.insert(record.guard_id, Vec::new());
        }
        mapped_records
            .get_mut(&record.guard_id)
            .unwrap()
            .push(record.clone());
    }
    let mut guard_reports: Vec<GuardReport> = Vec::new();
    for (guard_id, guard_records) in &mapped_records {
        guard_reports.push(GuardReport {
            guard_id: *guard_id,
            records: guard_records.to_vec(),
        })
    }
    return guard_reports;
}
