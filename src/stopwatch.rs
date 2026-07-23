use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Stopwatch {
    started_at: Option<Instant>,
    stopwatch: Duration,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            started_at: None,
            stopwatch: Duration::ZERO,
        }
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
        let secs = self.elapsed().as_secs();

        let hours = secs / 3600;
        let minutes = (secs % 3600) / 60;
        let seconds = secs % 60;

        format!("{hours:02}:{minutes:02}:{seconds:02}")
    }
}
