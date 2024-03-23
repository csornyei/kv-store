use std::str::FromStr;

pub enum PersistenceType {
    InMemory,
    JsonFile,
}

impl FromStr for PersistenceType {
    type Err = String;

    fn from_str(s: &str) -> Result<PersistenceType, String> {
        match s {
            "in_memory" => Ok(PersistenceType::InMemory),
            "json" => Ok(PersistenceType::JsonFile),
            _ => Err("Invalid persistence type!".to_string()),
        }
    }
}

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
}
