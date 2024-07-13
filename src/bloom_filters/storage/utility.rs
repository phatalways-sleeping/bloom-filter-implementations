use std::mem::size_of;

use crate::bloom_filters::{SmallestIntType, SupportedFloatingPointType};

pub(super) fn read_bit_coordinates(idx: usize) -> (usize, usize) {
    let element = idx / (size_of::<SmallestIntType>() * 8);
    let offset = idx % (size_of::<SmallestIntType>() * 8);
    return (element, offset);
}

pub(super) fn calculate_storage_capacity_based_on_total_bits(total_bits: usize) -> usize {
    ((total_bits as SupportedFloatingPointType)
        / ((size_of::<SmallestIntType>() * 8) as SupportedFloatingPointType))
        .ceil() as usize
}

#[cfg(test)]
mod test {
    #[test]
    fn should_provide_correct_capacity() {
        let test_cases = vec![(1, 1), (8, 1), (9, 2), (16, 2), (17, 3), (24, 3)];

        for (total_bits, expected_capacity) in test_cases {
            let capacity = super::calculate_storage_capacity_based_on_total_bits(total_bits);
            assert_eq!(capacity, expected_capacity);
        }
    }

    #[test]
    fn should_provide_correct_coordinates() {
        let test_cases = vec![
            (0, (0, 0)),
            (1, (0, 1)),
            (7, (0, 7)),
            (8, (1, 0)),
            (9, (1, 1)),
            (15, (1, 7)),
            (16, (2, 0)),
            (17, (2, 1)),
            (23, (2, 7)),
        ];

        for (idx, expected_coordinates) in test_cases {
            let coordinates = super::read_bit_coordinates(idx);
            assert_eq!(coordinates, expected_coordinates);
        }
    }
}
