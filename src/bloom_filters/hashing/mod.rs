use builder::Builder;
use strategy::Hashing;

mod builder;
mod seed;
pub mod strategy;
mod utility;

pub(crate) struct HashManager {
    num_of_hash_funcs: usize,
    total_bits: usize,
    strategy: Box<dyn Hashing>,
}

impl HashManager {
    pub(crate) fn builder() -> Builder {
        Builder::default()
    }

    pub(crate) fn hash(&self, entry: &dyn ToString) -> Vec<usize> {
        (0..self.num_of_hash_funcs)
            .map(|idx| self.strategy.hash(idx, self.total_bits, entry))
            .collect()
    }
}
