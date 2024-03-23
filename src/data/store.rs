use std::collections::HashMap;

use crate::data::data_value::Data;

use super::{data_value::DataValue, key::Key, DataTypes};

pub trait StoreManager: Data {
    fn get_name(&self) -> String;

    fn set_store(&mut self, store_name: Key) -> Result<String, String>;

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
            return self.set_value(key, value, data_type);
        }

        let store = key.store.clone().unwrap();
        let store: &mut Store = self.get_store(store)?;

        let key = key.get_next_key();

        store.set(key, value, data_type)
    }

    pub fn get(&mut self, key: Key) -> Result<String, String> {
        if key.is_value_key() {
            return self.get_value(key);
        }

        let store = key.store.clone().unwrap();
        let store: &mut Store = self.get_store(store)?;

        let key = key.get_next_key();

        store.get(key)
    }
}

impl Data for Store {
    fn get_type(&self) -> DataTypes {
        DataTypes::STORE
    }

    fn get_value(&self, key: Key) -> Result<String, String> {
        if key.is_value_key() {
            let data_key = key.key.clone().unwrap();

            let data = self.data.get(&data_key);

            if data.is_none() {
                return Err("Key not found".to_string());
            }

            return data.unwrap().get_value(key);
        } else {
            return Err("Invalid key".to_string());
        }
    }

    fn set_value(
        &mut self,
        key: Key,
        value: String,
        data_type: DataTypes,
    ) -> Result<String, String> {
        self.data
            .insert(key.key.clone().unwrap(), DataValue::new(value, data_type)?);
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

    fn set_store(&mut self, store_name: Key) -> Result<String, String> {
        if store_name.is_value_key() {
            let store_key = store_name.key.unwrap();
            if self.stores.contains_key(&store_key) {
                return Err("Key already exists".to_string());
            }
            let new_store = Store::new(store_key.clone());

            self.stores.insert(store_key, new_store);

            return Ok("OK".to_string());
        } else {
            let store_key = store_name.store.clone().unwrap();
            let store = self.get_store(store_key.clone())?;
            let store_name = store_name.get_next_key();
            return store.set_store(store_name);
        }
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
