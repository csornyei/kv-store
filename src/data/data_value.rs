use serde::{Deserialize, Serialize};

use crate::data::DataTypes;

use super::key::Key;

pub trait Data {
    fn get_type(&self) -> DataTypes;

    fn set_value(
        &mut self,
        key: Key,
        value: String,
        data_type: DataTypes,
    ) -> Result<String, String>;

    fn get_value(&self, key: Key) -> Result<String, String>;

    fn del_value(&mut self, key: &Key) -> Result<String, String>;
}

#[derive(Serialize, Deserialize)]
pub struct DataValue {
    pub value: String,
    data_type: DataTypes,
}

impl DataValue {
    pub fn new(value: String, data_type: DataTypes) -> Result<DataValue, String> {
        data_type.validate_data(&value)?;
        Ok(DataValue { value, data_type })
    }
}

impl Data for DataValue {
    fn get_type(&self) -> DataTypes {
        self.data_type.clone()
    }

    fn set_value(
        &mut self,
        key: Key,
        value: String,
        data_type: DataTypes,
    ) -> Result<String, String> {
        if data_type != self.data_type {
            return Err("Invalid data type".to_string());
        }
        self.value = value;
        Ok(format!("{} set", key.to_str()))
    }

    fn get_value(&self, _key: Key) -> Result<String, String> {
        Ok(self.value.clone())
    }

    fn del_value(&mut self, _key: &Key) -> Result<String, String> {
        self.value = "".to_string();
        Ok("Deleted".to_string())
    }
}
