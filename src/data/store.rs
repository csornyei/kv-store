use std::collections::HashMap;

use crate::data::data_value::Data;

use super::{data_value::DataValue, DataTypes};

pub trait StoreManager: Data {
    fn get_name(&self) -> String;

    fn set_store(&mut self, store_name: String) -> Result<String, String>;

    fn list_keys(&self) -> Result<Vec<String>, String>;
}

pub struct Store {
    name: String,
    pub data: HashMap<String, Box<dyn Data + Send + Sync>>,
}

impl Store {
    pub fn new(name: String) -> Store {
        Store {
            name,
            data: HashMap::new(),
        }
    }
}

impl Data for Store {
    fn get_type(&self) -> DataTypes {
        DataTypes::STORE
    }

    fn get(&self, key: String) -> Result<String, String> {
        self.data
            .get(&key)
            .map(|data| data.get(key.clone()))
            .unwrap_or(Err("Key not found".to_string()))
    }

    fn set_value(
        &mut self,
        key: String,
        value: String,
        data_type: DataTypes,
    ) -> Result<String, String> {
        if data_type != DataTypes::STRING {
            return Err("Invalid data type".to_string());
        }
        self.data
            .insert(key, Box::new(DataValue::new(value, data_type)?));
        Ok("OK".to_string())
    }

    fn del(&mut self, key: String) -> Result<String, String> {
        self.data.remove(&key);
        Ok("OK".to_string())
    }
}

impl StoreManager for Store {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_store(&mut self, store_name: String) -> Result<String, String> {
        let new_store = Store::new(store_name.clone());

        self.data.insert(store_name, Box::new(new_store));

        Ok("OK".to_string())
    }

    fn list_keys(&self) -> Result<Vec<String>, String> {
        Ok(self.data.keys().cloned().collect())
    }
}
