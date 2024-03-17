use crate::data::DataTypes;

#[derive(Debug)]
pub struct DataValue {
    pub value: String,
    _data_type: DataTypes,
}

impl DataValue {
    pub fn new(value: String, data_type: DataTypes) -> Result<DataValue, String> {
        data_type.validate_data(&value)?;
        Ok(DataValue {
            value,
            _data_type: data_type,
        })
    }
}
