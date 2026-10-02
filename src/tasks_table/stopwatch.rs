use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Stopwatch {
    started_at: Option<Instant>,
    stopwatch: Duration,
}

#[derive(Serialize, Deserialize)]
pub struct StopwatchData {
    pub elapsed_secs: u64,
    pub running: bool,
}

impl Stopwatch {
    pub fn to_data(&self) -> StopwatchData {
        StopwatchData {
            elapsed_secs: self.elapsed().as_secs(),
            running: self.running(),
        }
    }

    pub fn from_data(data: StopwatchData) -> Self {
        Stopwatch {
            started_at: None,
            stopwatch: Duration::from_secs(data.elapsed_secs),
        }
    }

    pub fn started(&self) -> bool {
        self.started_at.is_some() || self.stopwatch != Duration::ZERO
    }

    pub fn start(&mut self) {
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(start) = self.started_at.take() {
            self.stopwatch += start.elapsed();
        }
    }

    pub fn running(&self) -> bool {
        self.started_at.is_some()
    }

    pub fn elapsed(&self) -> Duration {
        match self.started_at {
            Some(start) => self.stopwatch + start.elapsed(),
            None => self.stopwatch,
        }
    }

    pub fn reset(&mut self) {
        self.started_at = None;
        self.stopwatch = Duration::ZERO;
    }

    pub fn formatted(&self) -> String {
        if !self.started() {
            return String::new();
        }

        let secs = self.elapsed().as_secs();

        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }
}
