use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::Path,
};

use super::{StorageService, StorageServiceError};

pub struct LocalStorage {
    path: String,
}

impl TryFrom<&str> for LocalStorage {
    type Error = StorageServiceError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let path = Path::new(value);

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

        Ok(Self {
            path: value.to_string(),
        })
    }
}

impl StorageService for LocalStorage {
    fn contains(&self, entry: &str) -> Result<bool, StorageServiceError> {
        let file = File::open(&self.path).map_err(|_| {
            StorageServiceError::NotFound(format!("Cannot open file at {}", self.path))
        })?;

        let mut lines_buffer = BufReader::new(file).lines();

        while let Some(line) = lines_buffer.next() {
            if let Ok(word) = line {
                if word == entry {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    fn save(&self, entry: String) -> Result<(), StorageServiceError> {
        let mut file = OpenOptions::new()
            .write(true)
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
            .write(true)
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

        let mut lines_buffer = BufReader::new(file).lines();

        let mut words: Vec<String> = vec![];

        while let Some(line) = lines_buffer.next() {
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
