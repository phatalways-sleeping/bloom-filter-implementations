use std::ops::Deref;

use super::utility::calculate_total_bits_based_on_max_size_and_tolerance;

use super::{capacity::Capacity, tolerance::Tolerance};

pub(super) struct StorageBits(usize);

impl Deref for StorageBits {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StorageBits {
    const MAXIMUM_BITS_ALLOWED_IN_STORAGE: usize = 512 * 1024 * 1024 * 1024;

    pub(super) fn try_from(
        capacity: &Capacity,
        tolerance: &Tolerance,
    ) -> Result<Self, &'static str> {
        let total_bits_needed = calculate_total_bits_based_on_max_size_and_tolerance(
            capacity.get_capacity(),
            tolerance.get_maximum_tolerance(),
        );

        if total_bits_needed >= Self::MAXIMUM_BITS_ALLOWED_IN_STORAGE {
            return Err("Too many bits needed for such tolerance and capacity. Overflow occurred");
        }

        Ok(Self(total_bits_needed))
    }
}

#[cfg(test)]
mod test {
    use crate::bloom_filters::configurations::{
        capacity::Capacity, storage_bits::StorageBits, tolerance::Tolerance,
    };

    #[test]
    fn should_return_ok_when_bits_needed_not_maximum_allowed() {
        let capacity = Capacity::try_from(1_000_000).unwrap();
        let tolerance = Tolerance::try_from(0.01).unwrap();
        let maybe_storage_bits = StorageBits::try_from(&capacity, &tolerance);
        assert!(maybe_storage_bits.is_ok());
    }
}
