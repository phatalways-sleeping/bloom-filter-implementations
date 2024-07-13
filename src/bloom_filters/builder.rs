use super::{hashing::strategy::Hashing, BloomFilter, BloomFilterError};

#[derive(Default)]
pub struct Builder {
    max_size: Option<usize>,
    max_tolerance: Option<f32>,
    strategy: Option<Box<dyn Hashing>>,
}

impl Builder {
    pub fn build(self) -> Result<BloomFilter, BloomFilterError> {
        BloomFilter::try_from(
            self.max_size.unwrap_or(1_000_000),
            self.max_tolerance.unwrap_or(0.01),
            self.strategy,
        )
    }

    pub fn with_max_size(self, max_size: usize) -> Self {
        Self {
            max_size: Some(max_size),
            max_tolerance: self.max_tolerance,
            strategy: self.strategy,
        }
    }

    pub fn with_max_tolerance(self, max_tolerance: f32) -> Self {
        Self {
            max_size: self.max_size,
            max_tolerance: Some(max_tolerance),
            strategy: self.strategy,
        }
    }

    pub fn with_strategy(self, strategy: Box<dyn Hashing>) -> Self {
        Self {
            max_size: self.max_size,
            max_tolerance: self.max_tolerance,
            strategy: Some(strategy),
        }
    }
}
