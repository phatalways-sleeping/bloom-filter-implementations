pub use storage_service::{local_storage::LocalStorage, StorageService, StorageServiceError};

use crate::bloom_filters::BloomFilter;

mod storage_service;

#[derive(Debug)]
pub enum SpellCheckerError {
    Storage(StorageServiceError),
    Initialization(String),
}

pub struct SpellChecker {
    buffer: BloomFilter,
    database: Box<dyn StorageService>,
}

pub struct SpellCheckerBuilder {
    buffer: Option<BloomFilter>,
    database: Option<Box<dyn StorageService>>,
}

impl Default for SpellCheckerBuilder {
    fn default() -> Self {
        Self {
            buffer: None,
            database: None,
        }
    }
}

impl SpellCheckerBuilder {
    pub fn build(self) -> Result<SpellChecker, SpellCheckerError> {
        if self.buffer.is_none() {
            return Err(SpellCheckerError::Initialization(String::from(
                "Buffer has not been initialized",
            )));
        }

        if self.database.is_none() {
            return Err(SpellCheckerError::Initialization(String::from(
                "Database has not been initialized",
            )));
        }

        let (buffer, database) = (self.buffer.unwrap(), self.database.unwrap());

        let words = database
            .retrieve_all()
            .map_err(SpellCheckerError::Storage)?;

        words.into_iter().for_each(|entry| buffer.insert(entry));

        Ok(SpellChecker::from(buffer, database))
    }

    pub fn with_buffer(self, buffer: BloomFilter) -> Self {
        Self {
            buffer: Some(buffer),
            database: self.database,
        }
    }

    pub fn with_database(self, database: Box<dyn StorageService>) -> Self {
        Self {
            buffer: self.buffer,
            database: Some(database),
        }
    }
}

impl SpellChecker {
    pub fn builder() -> SpellCheckerBuilder {
        SpellCheckerBuilder::default()
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
        return Ok(false);
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
