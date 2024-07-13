use fasthash::murmur;
use fnv::FnvHasher;
use std::hash::Hasher;

use super::seed::Seed;

pub trait Hashing {
    fn hash(&self, idx: usize, total_bits: usize, entry: &dyn ToString) -> usize;
}

// CustomizedDoubleHasing uses double hasing strategy, combining two hash functions: murmurhash, fvnl hash
// Formula: H(idx, hM, hF) = (hM + idx * hF  + idx * idx) mod total_bits
pub(crate) struct DefaultHashingStrategy(Seed);

impl From<Seed> for DefaultHashingStrategy {
    fn from(value: Seed) -> Self {
        Self(value)
    }
}

impl Hashing for DefaultHashingStrategy {
    fn hash(&self, idx: usize, total_bits: usize, entry: &dyn ToString) -> usize {
        let murmurhash_value = murmur::hash32(entry.to_string()) as usize;
        let fnvlhash_value = {
            let mut hasher = FnvHasher::with_key(self.0.get_seed());
            hasher.write(entry.to_string().as_bytes());
            hasher.finish() as usize
        };
        (murmurhash_value
            + idx.checked_mul(fnvlhash_value).unwrap_or(0)
            + idx.checked_mul(idx).unwrap_or(0))
            % total_bits
    }
}
