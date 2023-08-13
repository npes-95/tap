use std::fmt;
use std::time::Duration;

pub struct Notation {
    pub quarter: Duration,
    pub dotted_quarter: Duration,
    pub eighth: Duration,
    pub dotted_eighth: Duration,
    pub sixteenth: Duration,
    pub dotted_sixteenth: Duration,
    pub thirtysecond: Duration,
    pub dotted_thirtysecond: Duration,
}

impl Notation {
    pub fn new(pulse: Duration) -> Self {
        Self {
            quarter: pulse,
            dotted_quarter: pulse.div_f32(1.5),
            eighth: pulse.div_f32(2_f32),
            dotted_eighth: pulse.div_f32(3_f32),
            sixteenth: pulse.div_f32(4_f32),
            dotted_sixteenth: pulse.div_f32(6_f32),
            thirtysecond: pulse.div_f32(8_f32),
            dotted_thirtysecond: pulse.div_f32(12_f32),
        }
    }
}

impl fmt::Display for Notation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "quarter: {} ms
            \rdotted quarter: {} ms
            \reighth: {} ms
            \rdotted eighth: {} ms
            \rsixteenth: {} ms
            \rdotted sixteenth: {} ms
            \rthirtysecond: {} ms
            \rdotted thirtysecond: {} ms",
            self.quarter.as_millis(),
            self.dotted_quarter.as_millis(),
            self.eighth.as_millis(),
            self.dotted_eighth.as_millis(),
            self.sixteenth.as_millis(),
            self.dotted_sixteenth.as_millis(),
            self.thirtysecond.as_millis(),
            self.dotted_thirtysecond.as_millis()
        )
    }
}

pub struct Bpm {
    pub value: f32,
}

const MINUTE_AS_MS: f32 = 60000_f32;

impl Bpm {
    pub fn new(pulse: Duration) -> Self {
        Self {
            value: MINUTE_AS_MS / pulse.as_millis() as f32,
        }
    }
}

impl fmt::Display for Bpm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.value)
    }
}
