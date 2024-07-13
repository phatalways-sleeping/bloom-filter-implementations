use capacity::Capacity;
use storage_bits::StorageBits;
use tolerance::Tolerance;

use super::SupportedFloatingPointType;

mod capacity;
mod storage_bits;
mod tolerance;
mod utility;

pub(crate) trait Configurable {
    fn get_max_tolerance(&self) -> SupportedFloatingPointType;
    fn get_total_bits(&self) -> usize;
}

#[derive(Debug)]
pub enum ConfigError {
    Validation(String),
    Dependency(String),
}

pub(crate) struct Configuration {
    tolerance: Tolerance,
    capacity: Capacity,
    storage_bits: StorageBits,
}

impl Configuration {
    pub(crate) fn try_from(
        max_tolerance: SupportedFloatingPointType,
        max_size: usize,
    ) -> Result<Self, ConfigError> {
        let (capacity, tolerance) = match (
            Capacity::try_from(max_size),
            Tolerance::try_from(max_tolerance),
        ) {
            (Ok(capacity), Ok(tolerance)) => (capacity, tolerance),
            (Err(message), _) => return Err(ConfigError::Dependency(String::from(message))),
            (_, Err(message)) => return Err(ConfigError::Validation(String::from(message))),
        };

        let storage_bits = match StorageBits::try_from(&capacity, &tolerance) {
            Ok(value) => value,
            Err(message) => return Err(ConfigError::Dependency(String::from(message))),
        };

        Ok(Self {
            tolerance,
            capacity,
            storage_bits,
        })
    }

    pub(crate) fn increase_unique_entry_count(&self) {
        self.capacity.increase_size_by_one()
    }
}

impl Configurable for Configuration {
    fn get_max_tolerance(&self) -> SupportedFloatingPointType {
        self.tolerance.get_maximum_tolerance()
    }

    fn get_total_bits(&self) -> usize {
        *self.storage_bits
    }
}
