use crate::bloom_filters::SupportedFloatingPointType;

use super::{
    seed::Seed,
    strategy::{DefaultHashingStrategy, Hashing},
    utility::calculate_number_of_hash_functions_based_on_tolerance,
    HashManager,
};

#[derive(Default)]
pub(crate) struct Builder {
    max_tolerance: Option<SupportedFloatingPointType>,
    total_bits: Option<usize>,
    strategy: Option<Box<dyn Hashing>>,
}

impl Builder {
    pub(crate) fn with_tolerance(self, size: SupportedFloatingPointType) -> Self {
        Self {
            max_tolerance: Some(size),
            total_bits: self.total_bits,
            strategy: self.strategy,
        }
    }

    pub(crate) fn with_total_bits(self, total_bits: usize) -> Self {
        Self {
            max_tolerance: self.max_tolerance,
            total_bits: Some(total_bits),
            strategy: self.strategy,
        }
    }

    pub(crate) fn use_strategy(self, hash_strategy: Box<dyn Hashing>) -> Self {
        Self {
            max_tolerance: self.max_tolerance,
            total_bits: self.total_bits,
            strategy: Some(hash_strategy),
        }
    }

    pub(crate) fn build(self) -> Result<HashManager, &'static str> {
        let max_tolerance = match self.max_tolerance {
            Some(value) if value > 0.0 || value < 1.0 => value,
            Some(_) => return Err("Invalid maximum tolerance value"),
            None => return Err("Maximum tolerance is not provided"),
        };

        let strategy = match self.strategy {
            Some(value) => value,
            None => Box::new(DefaultHashingStrategy::from(Seed::default())),
        };

        let total_bits = match self.total_bits {
            Some(value) if value > 0 => value,
            Some(_) => return Err("Total bits cannot be zero"),
            None => return Err("Total bits is not provided"),
        };

        let num_of_hash_funcs =
            calculate_number_of_hash_functions_based_on_tolerance(max_tolerance);

        Ok(HashManager {
            num_of_hash_funcs,
            strategy,
            total_bits,
        })
    }
}
