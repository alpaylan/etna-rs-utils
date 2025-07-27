use std::fmt::Display;
use std::fmt::Formatter;

pub enum Status {
    Aborted(String),
    FoundBug(String),
    Finished
}

pub struct SamplingResult {
    property: String,
    status: Status,
    tests: u64,
    passed: u64,
    discarded: u64,
}

impl Display for SamplingResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let status_str = match &self.status {
            Status::FoundBug(_) => "FoundBug",
            Status::Aborted(_) => "Aborted",
            Status::Finished => "Finished",
        };
        let counterexample = match &self.status {
            Status::FoundBug(msg) => format!(", \"counterexample\": \"{}\"", msg),
            _ => String::new(),
        };
        let error = match &self.status {
            Status::Aborted(msg) => format!(", \"error\": \"{}\"", msg),
            _ => String::new(),
        };
        write!(
            f,
            "{{ \"property\": \"{}\", \"status\": \"{}\", \"tests\": {}, \"passed\": {}, \"discarded\": {}{}{}}}",
            self.property, status_str, self.tests, self.passed, self.discarded,
            counterexample, error
        )
    }
}