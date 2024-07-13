pub(super) struct Seed(u64);

impl Seed {
    const DEFAULT_SEED: u64 = 29;

    pub(super) fn get_seed(&self) -> u64 {
        self.0
    }
}

impl Default for Seed {
    fn default() -> Self {
        Self(Self::DEFAULT_SEED)
    }
}

impl From<u64> for Seed {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod test {
    use super::Seed;

    #[test]
    fn create_with_default_seed_if_none() {
        let seed = Seed::default();

        assert_eq!(seed.get_seed(), Seed::DEFAULT_SEED);
    }

    #[test]
    fn create_with_customized_seed_if_passed() {
        let seed = Seed::from(37);

        assert_eq!(seed.get_seed(), 37);
    }
}
