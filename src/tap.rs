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

    pub fn bpm(&self) -> Result<u16, ()> {
        if self.count > 1 {
            Ok(60000 / (self.cumulative_interval.as_millis() / (self.count - 1) as u128) as u16)
        } else {
            Err(())
        }
    }

    pub fn count(&self) -> u16 {
        self.count
    }

    pub fn last_interval(&self) -> Duration {
        self.last_tap.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use std::{ops::Mul, thread::sleep};

    #[test]
    fn init() {
        let t = Tap::new();
        assert_eq!(t.count, 0);
        assert_eq!(t.cumulative_interval, Duration::new(0, 0));
    }

    #[test]
    fn tap() {
        let interval = Duration::from_millis(100);
        let mut t = Tap::new();

        t.tap();
        sleep(interval);
        t.tap();

        assert_eq!(t.count, 2);

        // testing with time intervals is a bit flaky, so approx values will do
        assert_approx_eq!(
            t.cumulative_interval.as_secs_f32(),
            interval.mul((t.count - 1).into()).as_secs_f32(),
            0.02
        );
        assert!(t.bpm().unwrap() > 570 as u16);
        assert!(t.bpm().unwrap() < 630 as u16);
    }

    #[test]
    fn not_enough_taps() {
        let t = Tap::new();

        assert_eq!(t.bpm(), Err(()));
    }
}
