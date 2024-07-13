use seed::Seed;
use strategy::{DefaultHashingStrategy, Hashing};
use utility::calculate_number_of_hash_functions_based_on_tolerance;

use super::SupportedFloatingPointType;

mod seed;
pub mod strategy;
mod utility;

pub(crate) struct HashManager {
    num_of_hash_funcs: usize,
    total_bits: usize,
    strategy: Box<dyn Hashing>,
}

impl HashManager {
    pub(crate) fn builder() -> HashManagerBuilder {
        HashManagerBuilder::default()
    }

    pub(crate) fn compute(&self, entry: &dyn ToString) -> Vec<usize> {
        (0..self.num_of_hash_funcs)
            .map(|idx| self.strategy.hash(idx, self.total_bits, entry))
            .collect()
    }
}

pub(crate) struct HashManagerBuilder {
    max_tolerance: Option<SupportedFloatingPointType>,
    total_bits: Option<usize>,
    strategy: Option<Box<dyn Hashing>>,
}

impl Default for HashManagerBuilder {
    fn default() -> Self {
        Self {
            max_tolerance: None,
            strategy: None,
            total_bits: None,
        }
    }
}

impl HashManagerBuilder {
    pub(super) fn with_tolerance(self, size: SupportedFloatingPointType) -> Self {
        Self {
            max_tolerance: Some(size),
            total_bits: self.total_bits,
            strategy: self.strategy,
        }
    }

    pub(super) fn with_total_bits(self, total_bits: usize) -> Self {
        Self {
            max_tolerance: self.max_tolerance,
            total_bits: Some(total_bits),
            strategy: self.strategy,
        }
    }

    pub(super) fn use_strategy(self, hash_strategy: Box<dyn Hashing>) -> Self {
        Self {
            max_tolerance: self.max_tolerance,
            total_bits: self.total_bits,
            strategy: Some(hash_strategy),
        }
    }

    pub(super) fn build(self) -> Result<HashManager, &'static str> {
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
