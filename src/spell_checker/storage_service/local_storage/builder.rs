use crate::spell_checker::StorageServiceError;

use super::LocalStorage;

#[derive(Default)]
pub struct Builder {
    storage_location: Option<String>,
}

impl Builder {
    pub fn build(self) -> Result<LocalStorage, StorageServiceError> {
        let storage_location = if let Some(value) = self.storage_location {
            value
        } else {
            return Err(StorageServiceError::NotFound(String::from(
                "Cannot find the location",
            )));
        };

        LocalStorage::try_from(storage_location)
    }

    pub fn with_storage_location(self, storage_location: &str) -> Self {
        Self {
            storage_location: Some(storage_location.to_string()),
        }
    }
}
