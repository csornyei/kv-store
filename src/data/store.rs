use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::data::data_value::Data;

use super::{data_value::DataValue, key::Key, DataTypes};

pub trait StoreManager: Data {
    fn get_name(&self) -> String;

    fn set_store(&mut self, store_name: Key) -> Result<String, String>;

    fn list_keys(&self) -> Result<String, String>;

    fn get_store(&mut self, store_name: Key) -> Result<&mut Store, String>;

    fn del_store(&mut self, store_name: &Key) -> Result<String, String>;
}

#[derive(Serialize, Deserialize)]
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

        let store: &mut Store = self.get_store(key.get_store_key())?;

        let key = key.get_next_key();

        store.set(key, value, data_type)
    }

    pub fn get(&mut self, key: Key) -> Result<String, String> {
        if key.is_value_key() {
            return self.get_value(key);
        }

        let store: &mut Store = self.get_store(key.get_store_key())?;

        let key = key.get_next_key();

        store.get(key)
    }

    pub fn del(&mut self, key: Key) -> Result<String, String> {
        if key.is_value_key() {
            return match self.del_value(&key) {
                Ok(_) => Ok("OK".to_string()),
                Err(_) => match self.del_store(&key) {
                    Ok(_) => Ok("OK".to_string()),
                    Err(_) => Err("Key not found".to_string()),
                },
            };
        }

        let store: &mut Store = self.get_store(key.get_store_key())?;

        let key = key.get_next_key();

        store.del(key)
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

    fn del_value(&mut self, key: &Key) -> Result<String, String> {
        let value_key = key.key.clone().unwrap();
        if self.data.contains_key(&value_key) {
            self.data.remove(&value_key);
            return Ok("OK".to_string());
        }
        return Err("Key not found".to_string());
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
                return Err(format!("Key already exists: {}", store_key));
            }
            let new_store = Store::new(store_key.clone());

            self.stores.insert(store_key, new_store);

            return Ok("OK".to_string());
        } else {
            let store = self.get_store(store_name.get_store_key())?;
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

    fn get_store(&mut self, store_name: Key) -> Result<&mut Store, String> {
        if store_name.is_value_key() {
            let store_key = store_name.key.clone().unwrap();
            if self.stores.contains_key(&store_key) {
                return Ok(self.stores.get_mut(&store_key).unwrap());
            }
            return Err("Key not found".to_string());
        }
        let store_key = store_name.get_store_key();
        let store = self.get_store(store_key)?;
        let store_name = store_name.get_next_key();
        return store.get_store(store_name);
    }

    fn del_store(&mut self, store_name: &Key) -> Result<String, String> {
        let store_key = store_name.key.clone().unwrap();
        if self.stores.contains_key(&store_key) {
            self.stores.remove(&store_key);
            return Ok("OK".to_string());
        }
        return Err("Key not found".to_string());
    }
}
