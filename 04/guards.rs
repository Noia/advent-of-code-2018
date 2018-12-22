#[macro_use] extern crate scan_fmt;

use std::string::String;
use std::collections::LinkedList;
use std::cmp;
use std::cmp::{Ordering, PartialOrd, Ord};
use std::fmt;
use std::fmt::Display;
use std::num::Wrapping;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Event {
    BEGIN_SHIFT,
    WAKE_UP,
    FALL_ASLEEP
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GuardState {
    AWAKE,
    ASLEEP
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RecordFragment {
    pub date_time: String,
    pub date: String,
    pub time: String,
    pub guard_id: Option<u32>,
    pub event: Event
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct GuardStateSpan {
    pub state: GuardState,
    pub start: u32,
    pub end: u32
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DatedGuardRecord {
    pub date: String,
    pub guard_id: u32,
    pub events: Vec<GuardStateSpan>
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

impl RecordFragment {
    pub fn new(from: &str) -> RecordFragment {
        // Hour is always going to be 00, but we track it anyway for correctness.
        let (date_time, record_value) = scan_fmt!(
            from,
            // [1518-03-19 00:00]
            "[{/\\d+-\\d+-\\d+ \\d+:\\d+/}] {/.*/}",
            String, String
        );
        let date_time_str = date_time.unwrap();
        let (date, time) = scan_fmt!(
            &date_time_str,
            // [1518-03-19 00:00]
            "{/\\d+-\\d+-\\d+} {\\d+:\\d+/}",
            String, String
        );
        let record_raw = record_value.unwrap();
        let record: RecordFragment;
        match record_raw.as_ref() {
            "falls asleep" => {
                record = RecordFragment{
                        date_time: date_time_str,
                        date: date.unwrap(),
                        time: time.unwrap(),
                        guard_id: None,
                        event: Event::FALL_ASLEEP
                };
            },
            "wakes up" => {
                record = RecordFragment{
                    date_time: date_time_str,
                    date: date.unwrap(),
                    time: time.unwrap(),
                    guard_id: Option::None,
                    event: Event::WAKE_UP
                };
             },
            _ => {
                let raw_guard_id = scan_fmt!(
                    &record_raw,
                    "Guard #{d} begins shift",
                    u32
                );
                record = RecordFragment{
                        date_time: date_time_str,
                        date: date.unwrap(),
                        time: time.unwrap(),
                        guard_id: Some(raw_guard_id.unwrap()),
                        event: Event::BEGIN_SHIFT
                };
            }
        }
        return record;
    }
}
