#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    pub fn with_min(&self, min: f64) -> Self {
        Self { min, max: self.max }
    }

    pub fn with_max(&self, max: f64) -> Self {
        Self { min: self.min, max }
    }
}
