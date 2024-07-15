use crate::{bloom_filters::BloomFilter, spell_checker::StorageService};

use super::{DetectError, PasswordDetector};

#[derive(Default)]
pub struct Builder {
    buffer: Option<BloomFilter>,
    database: Option<Box<dyn StorageService>>,
}

impl Builder {
    pub fn build(self) -> Result<PasswordDetector, DetectError> {
        if self.database.is_none() {
            return Err(DetectError::Initialize(String::from(
                "Storage has not been initialized",
            )));
        }

        if self.buffer.is_none() {
            return Err(DetectError::Initialize(String::from(
                "Buffer has not been initialized",
            )));
        }

        Ok(PasswordDetector {
            buffer: self.buffer.unwrap(),
            database: self.database.unwrap(),
        })
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
