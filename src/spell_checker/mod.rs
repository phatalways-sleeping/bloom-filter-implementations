use builder::Builder;

pub use storage_service::{local_storage::LocalStorage, StorageService, StorageServiceError};

use crate::bloom_filters::BloomFilter;

mod storage_service;

mod builder;

#[derive(Debug)]
pub enum SpellCheckerError {
    Storage(StorageServiceError),
    Initialization(String),
}

pub struct SpellChecker {
    buffer: BloomFilter,
    database: Box<dyn StorageService>,
}

impl SpellChecker {
    pub fn builder() -> Builder {
        Builder::default()
    }

    fn from(buffer: BloomFilter, database: Box<dyn StorageService>) -> Self {
        Self { buffer, database }
    }

    pub fn check_spelling_of(&self, entry: &str) -> Result<bool, SpellCheckerError> {
        if self.buffer.contains(entry) {
            return self
                .database
                .contains(entry)
                .map_err(SpellCheckerError::Storage);
        }
        Ok(false)
    }

    pub fn update_knowledge(&self, entries: Vec<String>) -> Result<(), SpellCheckerError> {
        self.database
            .save_bulk(
                entries
                    .into_iter()
                    .map(|entry| entry.trim().to_string())
                    .collect(),
            )
            .map_err(SpellCheckerError::Storage)
    }
}
