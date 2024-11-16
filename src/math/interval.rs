use core::panic;

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    Between { low: f64, high: f64 },
    LessThan(f64),
    GreaterThan(f64),
    Unbounded,
}

impl Interval {
    pub fn interval(lo: f64, hi: f64) -> Self {
        if lo > hi {
            panic!("Low should not be greater than high!");
        }
        Self::Between { low: lo, high: hi }
    }

    pub fn contains(self, x: f64) -> bool {
        match self {
            Interval::Between { low, high } => low < x && x < high,
            Interval::LessThan(high) => x < high,
            Interval::GreaterThan(low) => x > low,
            Interval::Unbounded => true,
        }
    }

    pub fn clamp_max(self, max: f64) -> Self {
        match self {
            Interval::Between { low, high } => {
                let new_high = high.min(max);
                if low > new_high {
                    panic!("Low should not be greater than high after clamping!");
                }
                Interval::Between {
                    low,
                    high: new_high,
                }
            }
            Interval::LessThan(high) => {
                let new_high = high.min(max);
                if new_high <= 0.0 {
                    panic!("High should be greater than 0 after clamping!");
                }
                Interval::LessThan(new_high)
            }
            Interval::GreaterThan(low) => Interval::GreaterThan(low),
            Interval::Unbounded => Interval::LessThan(max),
        }
    }

    pub fn clamp_min(self, min: f64) -> Self {
        match self {
            Interval::Between { low, high } => {
                let new_low = low.max(min);
                if new_low > high {
                    panic!("Low should not be greater than high after clamping!");
                }
                Interval::Between { low: new_low, high }
            }
            Interval::LessThan(high) => Interval::LessThan(high),
            Interval::GreaterThan(low) => {
                let new_low = low.max(min);
                if new_low >= 0.0 {
                    panic!("Low should be less than 0 after clamping!");
                }
                Interval::GreaterThan(new_low)
            }
            Interval::Unbounded => Interval::GreaterThan(min),
        }
    }
}
