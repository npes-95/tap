use std::time::{Duration, Instant};

pub struct Tap {
    count: u16,
    cumulative_interval: Duration,
    last_tap: Instant,
}

impl Tap {
    pub fn new() -> Self {
        Tap {
            count: 0,
            cumulative_interval: Duration::new(0, 0),
            last_tap: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.cumulative_interval = Duration::new(0, 0);
        self.last_tap = Instant::now();
    }

    pub fn tap(&mut self) {
        self.count += 1;

        if self.count > 0 {
            self.cumulative_interval += self.last_tap.elapsed();
        }

        self.last_tap = Instant::now();
    }

    pub fn bpm(&self) -> u16 {
        60000 / (self.cumulative_interval.as_millis() / self.count as u128) as u16
    }

    pub fn count(&self) -> u16 {
        self.count
    }

    pub fn last_interval(&self) -> Duration {
        self.last_tap.elapsed()
    }
}
