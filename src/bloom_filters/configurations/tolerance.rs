use crate::bloom_filters::SupportedFloatingPointType;

pub(crate) struct Tolerance(SupportedFloatingPointType);

impl Tolerance {
    pub(super) fn get_maximum_tolerance(&self) -> SupportedFloatingPointType {
        self.0
    }
}

impl TryFrom<Option<SupportedFloatingPointType>> for Tolerance {
    type Error = &'static str;

    fn try_from(value: Option<SupportedFloatingPointType>) -> Result<Self, Self::Error> {
        let value = value.unwrap_or(0.01);

        if value <= 0.0 || value >= 1.0 {
            return Err("Tolerance must be within 0.0 and 1.0");
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod test {
    use crate::bloom_filters::configurations::tolerance::Tolerance;

    #[test]
    fn should_return_err_when_tolerance_is_invalid() {
        let invalid_tolerances = vec![-0.1, 1.1, 2.0, 0.0];

        for invalid_tolerance in invalid_tolerances {
            let maybe_tolerance = Tolerance::try_from(Some(invalid_tolerance));

            assert!(maybe_tolerance.is_err());
        }
    }

    #[test]
    fn should_accept_tolerance_if_within_valid_range() {
        let valid_tolerances = vec![0.01, 0.1, 0.5, 0.99];

        for valid_tolerance in valid_tolerances {
            let maybe_tolerance = Tolerance::try_from(Some(valid_tolerance));

            assert!(maybe_tolerance.is_ok());
        }
    }
}
