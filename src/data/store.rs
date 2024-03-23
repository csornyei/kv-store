use std::collections::HashMap;

use crate::data::data_value::Data;

use super::{data_value::DataValue, key::Key, DataTypes};

pub trait StoreManager: Data {
    fn get_name(&self) -> String;

    fn set_store(&mut self, store_name: String) -> Result<String, String>;

    fn list_keys(&self) -> Result<String, String>;

    fn get_store(&mut self, store_name: String) -> Result<&mut Store, String>;
}

pub struct Store {
    name: String,
    pub data: HashMap<String, DataValue>,
    pub stores: HashMap<String, Store>,
}

impl Store {
    pub fn new(name: String) -> Store {
        Store {
            name,
            data: HashMap::new(),
            stores: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: Key, value: String, data_type: DataTypes) -> Result<String, String> {
        if key.is_value_key() {
            return self.set_value(key.key.unwrap(), value, data_type);
        }

        let store = key.store.clone().unwrap();
        let store: &mut Store = self.get_store(store)?;

        let key = key.get_next_key();

        store.set(key, value, data_type)
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
        self.data.insert(key, DataValue::new(value, data_type)?);
        Ok("OK".to_string())
    }

    fn del(&mut self, key: String) -> Result<String, String> {
        if self.data.contains_key(key.as_str()) {
            self.data.remove(&key);
            return Ok("OK".to_string());
        }
        if self.stores.contains_key(key.as_str()) {
            self.stores.remove(&key);
            return Ok("OK".to_string());
        }
        Err("Key not found".to_string())
    }
}

impl StoreManager for Store {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_store(&mut self, store_name: String) -> Result<String, String> {
        if self.data.contains_key(&store_name) {
            return Err("Key already exists".to_string());
        }

        let new_store = Store::new(store_name.clone());

        self.stores.insert(store_name, new_store);

        Ok("OK".to_string())
    }

    fn list_keys(&self) -> Result<String, String> {
        Ok(self
            .stores
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .join("\n"))
    }

    fn get_store(&mut self, store_name: String) -> Result<&mut Store, String> {
        if self.stores.contains_key(&store_name) {
            return Ok(self.stores.get_mut(&store_name).unwrap());
        }
        Err("Store not found".to_string())
    }
}
