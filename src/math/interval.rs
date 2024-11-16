use core::panic;

#[derive(Debug, Clone, Copy)]
pub enum Interval {
    Between { low: f64, high: f64 },
    LessThan(f64),
    GreaterThan(f64),
    Unbounded,
}

impl Interval {
    pub fn contains(self, x: f64) -> bool {
        match self {
            Interval::Between { low, high } => low < x && x < high,
            Interval::LessThan(high) => x < high,
            Interval::GreaterThan(low) => x > low,
            Interval::Unbounded => true,
        }
    }

    pub fn clamp_high(self, new_high: f64) -> Self {
        match self {
            Interval::Between { low, high } => {
                let res_high = high.min(new_high);
                if low > res_high {
                    panic!("Low should not be greater than high after clamping!");
                }
                Interval::Between {
                    low,
                    high: res_high,
                }
            }
            Interval::LessThan(high) => {
                let new_high = high.min(new_high);
                if new_high <= 0.0 {
                    panic!("High should be greater than 0 after clamping!");
                }
                Interval::LessThan(new_high)
            }
            Interval::GreaterThan(low) => {
                if low > new_high {
                    panic!("Low should not be greater than high after clamping!");
                }
                Interval::Between {
                    low,
                    high: new_high,
                }
            }
            Interval::Unbounded => Interval::LessThan(new_high),
        }
    }

    pub fn clamp_low(self, new_low: f64) -> Self {
        match self {
            Interval::Between { low, high } => {
                let res_low = low.max(new_low);
                if res_low > high {
                    panic!("Low should not be greater than high after clamping!");
                }
                Interval::Between { low: res_low, high }
            }
            Interval::LessThan(high) => {
                if new_low > high {
                    panic!("Low should not be greater than high after clamping!");
                }
                Interval::Between { low: new_low, high }
            }
            Interval::GreaterThan(low) => {
                let new_low = low.max(new_low);
                if new_low >= 0.0 {
                    panic!("Low should be less than 0 after clamping!");
                }
                Interval::GreaterThan(new_low)
            }
            Interval::Unbounded => Interval::GreaterThan(new_low),
        }
    }
}
