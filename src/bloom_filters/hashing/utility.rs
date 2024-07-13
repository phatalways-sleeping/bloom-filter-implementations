use crate::bloom_filters::SupportedFloatingPointType;

pub(super) fn calculate_number_of_hash_functions_based_on_tolerance(
    max_tolerance: SupportedFloatingPointType,
) -> usize {
    let factor: SupportedFloatingPointType = 2.0;
    ((-max_tolerance.ln() / factor.ln()).ceil()) as usize
}

#[cfg(test)]
mod test {
    #[test]
    fn should_provide_correct_number_of_hash_functions() {
        let test_cases = vec![(0.01, 7), (0.1, 4), (0.2, 3), (0.3, 2), (0.001, 10)];

        for (tolerance, expected_number_of_hash_functions) in test_cases {
            let number_of_hash_functions =
                super::calculate_number_of_hash_functions_based_on_tolerance(tolerance);
            assert_eq!(number_of_hash_functions, expected_number_of_hash_functions);
        }
    }
}
