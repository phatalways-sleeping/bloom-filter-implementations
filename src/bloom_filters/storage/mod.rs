use std::cell::RefCell;

use utility::{calculate_storage_capacity_based_on_total_bits, read_bit_coordinates};

mod utility;

pub(super) struct Storage(RefCell<Vec<u8>>);

impl Storage {
    pub(super) fn try_from(total_bits: usize) -> Result<Self, &'static str> {
        if total_bits == 0 {
            return Err("Total bits must be positive");
        }

        let capacity = calculate_storage_capacity_based_on_total_bits(total_bits);

        Ok(Self(RefCell::new(vec![0; capacity])))
    }

    pub(super) fn write_bit_at(&self, idx: usize) -> bool {
        let (element, bits) = read_bit_coordinates(idx);
        let mask = 1 << bits;
        let entry = self.0.borrow()[element];
        if entry & mask != 0 {
            return false;
        }
        self.0.borrow_mut()[element] = entry | mask;
        true
    }

    pub(super) fn read_bit_at(&self, idx: usize) -> u8 {
        let (element, bits) = read_bit_coordinates(idx);
        return self.0.borrow()[element] & (1 << bits);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn should_return_err_when_total_bits_is_zero() {
        let storage = super::Storage::try_from(0);
        assert!(storage.is_err());
    }

    #[test]
    fn should_return_storage_when_total_bits_is_positive() {
        let storage = super::Storage::try_from(1);
        assert!(storage.is_ok());
    }

    #[test]
    fn should_write_bit_at_given_index() {
        let storage = super::Storage::try_from(8).unwrap();
        let is_written = storage.write_bit_at(0);
        assert!(is_written);
    }

    #[test]
    fn should_not_write_bit_at_given_index_if_already_written() {
        let storage = super::Storage::try_from(8).unwrap();
        storage.write_bit_at(0);
        let is_written = storage.write_bit_at(0);
        assert!(!is_written);
    }

    #[test]
    fn should_read_bit_at_given_index() {
        let storage = super::Storage::try_from(8).unwrap();
        storage.write_bit_at(0);
        let bit = storage.read_bit_at(0);
        assert_eq!(bit, 1);
        let bit = storage.read_bit_at(1);
        assert_eq!(bit, 0);
    }
}
