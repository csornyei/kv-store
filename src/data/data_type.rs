use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Debug)]
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
