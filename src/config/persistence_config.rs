use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    data::Store,
    persistence::{Persistence, PersistenceType},
};

#[derive(Serialize, Deserialize, Clone)]

pub struct PersistenceConfig(pub HashMap<PersistenceType, Persistence>);

impl Default for PersistenceConfig {
    fn default() -> Self {
        let mut persistence = HashMap::new();
        persistence.insert(PersistenceType::InMemory, Persistence::new_in_memory());

        PersistenceConfig(persistence)
    }
}

impl PersistenceConfig {
    pub fn get_persistence(&self, persistence_type: PersistenceType) -> Option<Persistence> {
        self.0.get(&persistence_type).clone().cloned()
    }

    pub fn get_json_file(&self) -> Option<Persistence> {
        self.0.get(&PersistenceType::JsonFile).clone().cloned()
    }

    pub fn get_logger(&self) -> Option<Persistence> {
        self.0
            .get(&PersistenceType::AppendOnlyLogger)
            .clone()
            .cloned()
    }

    pub fn load_store(&self) -> Result<Store, String> {
        match self.get_json_file() {
            Some(persistence) => persistence.load_store(),
            None => {
                println!("No JSON file persistence found, creating new store");
                Persistence::new_in_memory().load_store()
            }
        }
    }
}
