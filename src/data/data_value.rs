use crate::data::DataTypes;

pub trait Data {
    fn get_type(&self) -> DataTypes;

    fn set(&mut self, key: String, value: String, data_type: DataTypes) -> Result<String, String>;

    fn get(&self, key: String) -> Result<String, String>;

    fn del(&mut self, key: String) -> Result<String, String>;
}

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

    fn set(&mut self, key: String, value: String, data_type: DataTypes) -> Result<String, String> {
        if data_type != self.data_type {
            return Err("Invalid data type".to_string());
        }
        self.value = value;
        Ok(format!("{} set", key))
    }

    fn get(&self, _key: String) -> Result<String, String> {
        Ok(self.value.clone())
    }

    fn del(&mut self, _key: String) -> Result<String, String> {
        self.value = "".to_string();
        Ok("Deleted".to_string())
    }
}
