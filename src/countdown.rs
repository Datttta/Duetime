use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Default)]
pub struct Countdown {
    ends_at: Option<Instant>,
    ends_at_timestamp: Option<i64>,
    remaining: Duration,
}

impl Countdown {
    pub fn new(duration: Duration) -> Self {
        Self {
            ends_at: None,
            ends_at_timestamp: None,
            remaining: duration,
        }
    }

    pub fn from_data(
        remaining: Duration,
        ends_at_timestamp: Option<i64>,
    ) -> Self {
        let Some(timestamp) = ends_at_timestamp else {
            return Self {
                ends_at: None,
                ends_at_timestamp: None,
                remaining,
            };
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        if timestamp <= now {
            return Self {
                ends_at: None,
                ends_at_timestamp: None,
                remaining: Duration::ZERO,
            };
        }

        let remaining = Duration::from_secs(
            (timestamp - now) as u64
        );

        Self {
            ends_at: Some(
                Instant::now() + remaining
            ),
            ends_at_timestamp: Some(timestamp),
            remaining,
        }
    }


    pub fn start(&mut self) {
        if self.ends_at.is_none() && self.remaining > Duration::ZERO {
            self.ends_at = Some(Instant::now() + self.remaining);

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            self.ends_at_timestamp = Some(
                timestamp + self.remaining.as_secs() as i64
            );
        }
    }

    pub fn pause(&mut self) {
        if let Some(ends_at) = self.ends_at.take() {
            self.remaining =
                ends_at.saturating_duration_since(Instant::now());

            self.ends_at_timestamp = None;
        }
    }

    pub fn remaining(&self) -> Duration {
        match self.ends_at {
            Some(ends_at) => {
                ends_at.saturating_duration_since(Instant::now())
            }
            None => self.remaining,
        }
    }

    pub fn running(&self) -> bool {
        self.ends_at.is_some()
    }

    pub fn ends_at_timestamp(&self) -> Option<i64> {
        self.ends_at_timestamp
    }
}
