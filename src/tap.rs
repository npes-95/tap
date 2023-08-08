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
        // ignore first interval (time between instantiation and first tap)
        if self.count > 0 {
            self.cumulative_interval += self.last_tap.elapsed();
        }

        self.count += 1;
        self.last_tap = Instant::now();
    }

    pub fn count(&self) -> u16 {
        self.count
    }

    pub fn average_interval(&self) -> Result<Duration, ()> {
        if self.count > 1 {
            Ok(self.cumulative_interval.div_f32((self.count - 1) as f32))
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let t = Tap::new();
        assert_eq!(t.count, 0);
        assert_eq!(t.cumulative_interval, Duration::new(0, 0));
    }

    #[test]
    fn not_enough_taps() {
        let t = Tap::new();

        assert_eq!(t.average_interval(), Err(()));
    }
}
