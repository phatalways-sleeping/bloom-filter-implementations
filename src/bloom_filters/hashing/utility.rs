use crate::bloom_filters::SupportedFloatingPointType;

pub(super) fn calculate_number_of_hash_functions_based_on_tolerance(
    max_tolerance: SupportedFloatingPointType,
) -> usize {
    let factor: SupportedFloatingPointType = 2.0;
    (-(max_tolerance.ln() / factor.ln()).ceil()) as usize
}
