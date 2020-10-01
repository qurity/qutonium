use std::time::{Duration, Instant};


use miniserde::Serialize;


#[derive(Copy, Clone, Debug, Serialize)]
pub enum TimeUnitType {
  Microseconds,
  Milliseconds,
  Nanoseconds,
  Seconds,
}


#[derive(Clone, Copy, Debug, Serialize)]
pub struct TimeUnit {
  pub micros: u64,
  pub millis: u64,
  pub nanos: u64,
  pub secs: u64,
}


#[derive(Clone, Copy)]
pub struct Time {
  pub start: Option<Instant>,
  pub end: Option<Duration>,
  pub unit: TimeUnit,
}


impl Time {
  pub fn new () -> Self {
    Time {
      end: None,
      start: None,
      unit: TimeUnit {
        micros: 0,
        millis: 0,
        nanos: 0,
        secs: 0,
      },
    }
  }

  pub fn start (mut self) -> Option<Instant> {
    self.start = Some(Instant::now());
    self.start
  }

  pub fn end (mut self) -> Option<Duration> {
    self.end = match self.start {
      Some(start) => Some(start.elapsed()),
      _ => None,
    };

    self.end
  }
}
