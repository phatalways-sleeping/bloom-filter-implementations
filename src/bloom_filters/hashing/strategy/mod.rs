use fasthash::murmur;
use fnv::FnvHasher;
use std::hash::Hasher;

use super::seed::Seed;

pub trait Hashing {
    fn hash(&self, idx: usize, total_bits: usize, entry: &dyn ToString) -> usize;
}

// DefaultHashingStrategy uses double hasing strategy, combining two hash functions: murmurhash, fvnl hash
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

        let mut hashed_value = murmurhash_value;

        let (result, overflowed) = idx.overflowing_mul(fnvlhash_value);

        if !overflowed {
            hashed_value = hashed_value.wrapping_add(result);
        } else {
            let remaining = (idx as u128 * fnvlhash_value as u128) % (usize::MAX as u128 + 1);
            hashed_value = hashed_value.wrapping_add(remaining as usize);
        }

        let (result, overflowed) = idx.overflowing_mul(idx);

        if !overflowed {
            hashed_value = hashed_value.wrapping_add(result);
        } else {
            let remaining = (idx as u128 * idx as u128) % (usize::MAX as u128 + 1);
            hashed_value = hashed_value.wrapping_add(remaining as usize);
        }

        hashed_value % total_bits
    }
}
