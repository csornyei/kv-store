use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::data::Store;

#[derive(Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum PersistenceType {
    InMemory,
    JsonFile,
    AppendOnlyLogger,
}

impl FromStr for PersistenceType {
    type Err = String;

    fn from_str(s: &str) -> Result<PersistenceType, String> {
        match s {
            "in_memory" => Ok(PersistenceType::InMemory),
            "json" => Ok(PersistenceType::JsonFile),
            "append_only_logger" => Ok(PersistenceType::AppendOnlyLogger),
            _ => Err("Invalid persistence type!".to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Persistence {
    persistence_type: PersistenceType,
    file_path: Option<String>,
}

impl Persistence {
    pub fn new_in_memory() -> Persistence {
        Persistence {
            persistence_type: PersistenceType::InMemory,
            file_path: None,
        }
    }

    pub fn new_json_file(file_path: String) -> Persistence {
        Persistence {
            persistence_type: PersistenceType::JsonFile,
            file_path: Some(file_path),
        }
    }

    pub fn get_type(&self) -> PersistenceType {
        self.persistence_type.clone()
    }

    pub fn get_path(&self) -> Option<String> {
        self.file_path.clone()
    }

    pub fn save_store(&self, data: &Store) -> Result<(), String> {
        match self.persistence_type {
            PersistenceType::JsonFile => {
                let json = serde_json::to_string(data).unwrap();
                match self.file_path.clone() {
                    Some(path) => std::fs::write(path, json).unwrap(),
                    None => return Err("No file path provided".to_string()),
                }
            }
            _ => return Err("Invalid persistence type".to_string()),
        }
        Ok(())
    }

    pub fn load_store(&self) -> Result<Store, String> {
        match self.persistence_type {
            PersistenceType::JsonFile => match self.file_path.clone() {
                Some(path) => {
                    let json = std::fs::read(path).unwrap();
                    if json.is_empty() {
                        return Ok(Store::new(".".to_string()));
                    }
                    let store: Store = serde_json::from_slice(&json).unwrap();
                    Ok(store)
                }
                None => Err("No file path provided".to_string()),
            },
            PersistenceType::InMemory => Ok(Store::new(".".to_string())),
            _ => Err("Invalid persistence type".to_string()),
        }
    }
}
