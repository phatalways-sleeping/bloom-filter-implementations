use crate::bloom_filters::SupportedFloatingPointType;

pub(super) fn calculate_total_bits_based_on_max_size_and_tolerance(
    max_size: usize,
    max_tolerance: SupportedFloatingPointType,
) -> usize {
    let factor: SupportedFloatingPointType = 2.0;
    (-((max_size as SupportedFloatingPointType) * max_tolerance.ln() / factor.ln() / factor.ln())
        .ceil()) as usize
}

#[cfg(test)]
mod test {
    use crate::bloom_filters::configurations::utility::calculate_total_bits_based_on_max_size_and_tolerance;

    #[test]
    fn should_return_correct_total_bits() {
        let test_cases = vec![
            (1_000_000, 0.01, 9585058),
            (1_000_000, 0.1, 4792529),
            (1_000_000, 0.5, 1442695),
            (1_000_000, 0.99, 20918),
        ];

        for (max_size, max_tolerance, expected_total_bits) in test_cases {
            let total_bits =
                calculate_total_bits_based_on_max_size_and_tolerance(max_size, max_tolerance);
            assert_eq!(total_bits, expected_total_bits);
        }
    }
}
