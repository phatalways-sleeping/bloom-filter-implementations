use configurations::{ConfigError, Configurable, Configuration};
use hashing::{strategy::Hashing, HashManager};
use storage::Storage;

mod configurations;
mod hashing;
mod storage;

type SmallestIntType = u8;
type SupportedFloatingPointType = f32;

#[derive(Debug)]
pub enum BloomFilterError {
    Configuration(ConfigError),
    Hashing(String),
    Storage(String),
}

pub struct BloomFilter {
    configuration: Configuration,
    manager: HashManager,
    storage: Storage,
}

impl BloomFilter {
    pub fn from(
        max_size: usize,
        max_tolerance: Option<f32>,
        strategy: Option<Box<dyn Hashing>>,
    ) -> Result<Self, BloomFilterError> {
        let configuration = Configuration::try_from(max_tolerance, max_size)
            .map_err(BloomFilterError::Configuration)?;

        let mut manager_builder = HashManager::builder()
            .with_tolerance(configuration.get_max_tolerance())
            .with_total_bits(configuration.get_total_bits());

        if let Some(strategy) = strategy {
            manager_builder = manager_builder.use_strategy(strategy);
        }

        let manager = manager_builder
            .build()
            .map_err(String::from)
            .map_err(BloomFilterError::Hashing)?;

        let storage = Storage::from(configuration.get_total_bits())
            .map_err(String::from)
            .map_err(BloomFilterError::Storage)?;

        Ok(Self {
            configuration,
            manager,
            storage,
        })
    }

    pub fn insert(&self, entry: String) {
        let positions = self.manager.compute(&entry);

        let is_new_entry = positions
            .into_iter()
            .map(|idx| self.storage.write_bit_at(idx))
            .fold(true, |acc, x| acc && x);

        if is_new_entry {
            self.configuration.increase_unique_entry_count();
        }
    }

    pub fn contains(&self, entry: &str) -> bool {
        let positions = self.manager.compute(&entry);

        positions
            .into_iter()
            .map(|idx| self.storage.read_bit_at(idx))
            .all(|bit| bit != 0)
    }
}
