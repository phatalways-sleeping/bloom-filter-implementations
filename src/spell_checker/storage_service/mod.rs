pub mod local_storage;

#[derive(Debug)]
pub enum StorageServiceError {
    NotFound(String),
    Unsupported(String),
    PermissionDenied(String),
    NetworkIssue(String),
}

pub trait StorageService {
    fn contains(&self, entry: &str) -> Result<bool, StorageServiceError>;
    fn save(&self, entry: String) -> Result<(), StorageServiceError>;
    fn save_bulk(&self, entries: Vec<String>) -> Result<(), StorageServiceError>;
    fn retrieve_all(&self) -> Result<Vec<String>, StorageServiceError>;
}
