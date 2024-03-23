use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum DataTypes {
    STRING,
    INT,
    FLOAT,
    BOOL,
    STORE,
}

impl FromStr for DataTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "STRING" => Ok(DataTypes::STRING),
            "INT" => Ok(DataTypes::INT),
            "FLOAT" => Ok(DataTypes::FLOAT),
            "BOOL" => Ok(DataTypes::BOOL),
            "STORE" => Ok(DataTypes::STORE),
            _ => Err("Invalid data type".to_string()),
        }
    }
}

impl Display for DataTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataTypes::STRING => write!(f, "STRING"),
            DataTypes::INT => write!(f, "INT"),
            DataTypes::FLOAT => write!(f, "FLOAT"),
            DataTypes::BOOL => write!(f, "BOOL"),
            DataTypes::STORE => write!(f, "STORE"),
        }
    }
}

impl DataTypes {
    pub fn validate_data(&self, value: &str) -> Result<(), String> {
        match self {
            DataTypes::STRING => Ok(()),
            DataTypes::INT => match value.parse::<i64>() {
                Ok(_) => Ok(()),
                Err(_) => Err("Invalid data type".to_string()),
            },
            DataTypes::FLOAT => match value.parse::<f64>() {
                Ok(_) => Ok(()),
                Err(_) => Err("Invalid data type".to_string()),
            },
            DataTypes::BOOL => match value.parse::<bool>() {
                Ok(_) => Ok(()),
                Err(_) => Err("Invalid data type".to_string()),
            },
            DataTypes::STORE => Ok(()),
        }
    }
}

#[cfg(test)]
mod data_types_tests {

    use super::*;

    #[test]
    fn test_data_type_string_to_data_types() {
        let data_type = DataTypes::from_str("STRING").unwrap();
        assert_eq!(data_type, DataTypes::STRING);
    }

    #[test]
    fn test_data_type_invalid_string_to_data_types() {
        let data_type = DataTypes::from_str("INVALID");
        assert_eq!(data_type, Err("Invalid data type".to_string()));
    }

    #[test]
    fn test_data_type_int_to_data_type() {
        let data_type = DataTypes::from_str("INT").unwrap();
        assert_eq!(data_type, DataTypes::INT);
    }

    #[test]
    fn test_data_type_float_to_data_type() {
        let data_type = DataTypes::from_str("FLOAT").unwrap();
        assert_eq!(data_type, DataTypes::FLOAT);
    }

    #[test]
    fn test_data_type_bool_to_data_type() {
        let data_type = DataTypes::from_str("BOOL").unwrap();
        assert_eq!(data_type, DataTypes::BOOL);
    }

    #[test]
    fn test_data_type_store_to_data_type() {
        let data_type = DataTypes::from_str("STORE").unwrap();
        assert_eq!(data_type, DataTypes::STORE);
    }

    #[test]
    fn test_data_type_validate_string() {
        let data_type = DataTypes::STRING;
        let value = "value";
        let result = data_type.validate_data(value);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_data_type_validate_int() {
        let data_type = DataTypes::INT;
        let value = "10";
        let result = data_type.validate_data(value);
        assert_eq!(result, Ok(()));

        let value = "invalid";
        let result = data_type.validate_data(value);
        assert_eq!(result, Err("Invalid data type".to_string()));
    }

    #[test]
    fn test_data_type_validate_float() {
        let data_type = DataTypes::FLOAT;
        let value = "10.5";
        let result = data_type.validate_data(value);
        assert_eq!(result, Ok(()));

        let value = "invalid";
        let result = data_type.validate_data(value);
        assert_eq!(result, Err("Invalid data type".to_string()));
    }

    #[test]
    fn test_data_type_validate_bool() {
        let data_type = DataTypes::BOOL;
        let value = "true";
        let result = data_type.validate_data(value);
        assert_eq!(result, Ok(()));

        let value = "invalid";
        let result = data_type.validate_data(value);
        assert_eq!(result, Err("Invalid data type".to_string()));
    }

    #[test]
    fn test_data_type_reparse_data_type() {
        let data_type = DataTypes::from_str("STRING").unwrap();
        let data_type_str = data_type.to_string();
        let data_type = DataTypes::from_str(&data_type_str).unwrap();
        assert_eq!(data_type, DataTypes::STRING);
    }
}
