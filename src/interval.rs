// double min, max;
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval {
            min,
            max,
        }
    }

    pub fn empty() -> Interval {
        Interval {
            min: f32::INFINITY,
            max: -f32::INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Interval {
            min: -f32::INFINITY,
            max: f32::INFINITY,
        }
    }

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
}