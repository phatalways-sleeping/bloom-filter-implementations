use builder::Builder;

use crate::{
    bloom_filters::BloomFilter,
    spell_checker::{StorageService, StorageServiceError},
};

mod builder;

pub struct PasswordDetector {
    buffer: BloomFilter,
    database: Box<dyn StorageService>,
}

#[derive(Debug)]
pub enum DetectError {
    Initialize(String),
    Storage(StorageServiceError),
    Dismiss,
    Approve,
}

impl PasswordDetector {
    pub fn builder() -> Builder {
        Builder::default()
    }
    
    pub fn verify(&self, password: &str) -> DetectError {
        match self.buffer.contains(password) {
            true => match self.database.contains(password) {
                Ok(value) => {
                    if value {
                        DetectError::Dismiss
                    } else {
                        DetectError::Approve
                    }
                }
                Err(err) => DetectError::Storage(err),
            },
            false => DetectError::Approve,
        }
    }
}
