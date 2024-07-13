use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

pub use builder::Builder;

use super::{StorageService, StorageServiceError};

mod builder;

pub struct LocalStorage {
    path: String,
}

impl LocalStorage {
    pub fn builder() -> Builder {
        Builder::default()
    }

    fn try_from(storage_loc: String) -> Result<Self, StorageServiceError> {
        let path = Path::new(&storage_loc);

        if !path.exists() {
            return Err(StorageServiceError::NotFound(String::from(
                "Incorrect path.",
            )));
        }

        if !path.is_file() {
            return Err(StorageServiceError::Unsupported(String::from(
                "This path does not lead to a file",
            )));
        }

        match path.extension() {
            Some(ext) if ext != "txt" => {
                return Err(StorageServiceError::Unsupported(String::from(
                    "This storage only supports text files",
                )))
            }
            None => {
                return Err(StorageServiceError::NotFound(String::from(
                    "No file extension found",
                )))
            }
            _ => {}
        }

        Ok(Self { path: storage_loc })
    }
}

impl StorageService for LocalStorage {
    fn contains(&self, entry: &str) -> Result<bool, StorageServiceError> {
        let file = File::open(&self.path).map_err(|_| {
            StorageServiceError::NotFound(format!("Cannot open file at {}", self.path))
        })?;

        let lines_buffer = BufReader::new(file).lines();

        for line in lines_buffer {
            let Ok(word) = line else { continue };
            if word == entry {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn save(&self, entry: String) -> Result<(), StorageServiceError> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.path)
            .map_err(|_| {
                StorageServiceError::NotFound(format!("Cannot open file at {}", self.path))
            })?;

        writeln!(&mut file, "{}", entry).map_err(|_| {
            StorageServiceError::PermissionDenied(String::from("Cannot write to the local file"))
        })
    }

    fn save_bulk(&self, entries: Vec<String>) -> Result<(), StorageServiceError> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.path)
            .map_err(|_| {
                StorageServiceError::NotFound(format!("Cannot open file at {}", self.path))
            })?;

        for entry in entries {
            writeln!(&mut file, "{}", entry).map_err(|_| {
                StorageServiceError::PermissionDenied(String::from(
                    "Cannot write to the local file",
                ))
            })?
        }

        Ok(())
    }

    fn retrieve_all(&self) -> Result<Vec<String>, StorageServiceError> {
        let file = File::open(&self.path).map_err(|_| {
            StorageServiceError::NotFound(format!("Cannot open file at {}", self.path))
        })?;

        let lines_buffer = BufReader::new(file).lines();

        let mut words: Vec<String> = vec![];

        for line in lines_buffer {
            match line {
                Ok(word) => words.push(word),
                Err(_) => {
                    return Err(StorageServiceError::PermissionDenied(String::from(
                        "Some lines have incorrect format",
                    )))
                }
            }
        }

        Ok(words)
    }
}
