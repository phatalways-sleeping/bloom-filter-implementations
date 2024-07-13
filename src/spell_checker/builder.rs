use crate::bloom_filters::BloomFilter;

use super::{SpellChecker, SpellCheckerError, StorageService};

#[derive(Default)]
pub struct Builder {
    buffer: Option<BloomFilter>,
    database: Option<Box<dyn StorageService>>,
}

impl Builder {
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
