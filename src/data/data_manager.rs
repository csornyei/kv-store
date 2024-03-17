use crate::commands::{Command, CommandNames};
use crate::data::auth_manager::{AuthManager, Permissions};
use crate::session::Session;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

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

#[derive(Debug)]
pub struct DataValue {
    value: String,
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

#[derive(Debug)]
pub struct DataManager {
    data: HashMap<String, DataValue>,
    auth_manager: AuthManager,
}

impl DataManager {
    pub fn new(admin_username: String, admin_password: String) -> Result<DataManager, String> {
        let auth_manager = AuthManager::new(admin_username, admin_password, 255)?;
        Ok(DataManager {
            data: HashMap::new(),
            auth_manager,
        })
    }

    pub fn handle_command(
        &mut self,
        cmd: Command,
        session: Session,
    ) -> Result<(String, Session), String> {
        match cmd.name {
            CommandNames::SET => {
                self.check_auth(&session, Permissions::SET)?;
                let key = cmd.args[0].clone();
                let value = cmd.args[1].clone();
                let data_type = DataTypes::from_str(&cmd.args[2])?;
                let result = self.set(key, value, data_type);
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::GET => {
                self.check_auth(&session, Permissions::GET)?;
                let key = cmd.args[0].clone();
                let result = self.get(key);
                match result {
                    Ok(value) => Ok((value, session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::DEL => {
                self.check_auth(&session, Permissions::DEL)?;
                let key = cmd.args[0].clone();
                let result = self.del(key);
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::AUTH => {
                let user_name = cmd.args[0].clone();
                let password = cmd.args[1].clone();
                let result = self.auth(user_name, password, session);
                match result {
                    Ok(session) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::GET_USER => {
                self.check_auth(&session, Permissions::USER_ADMIN)?;

                let username = cmd.args[0].clone();
                match self.auth_manager.get_user(username) {
                    Some(user) => Ok((user.to_string(), session)),
                    None => Err("User not found".to_string()),
                }
            }
            CommandNames::CREATE_USER => {
                self.check_auth(&session, Permissions::USER_ADMIN)?;
                let user_name = cmd.args[0].clone();
                let password = cmd.args[1].clone();
                let permissions = u8::from_str(&cmd.args[2]).unwrap();

                let permissions_to_set = Permissions::from_u8(permissions.clone());
                for p in permissions_to_set {
                    self.check_permission(&session, p)?;
                }

                let result = self.create_user(user_name, password, permissions);
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::DELETE_USER => {
                self.check_auth(&session, Permissions::USER_ADMIN)?;
                let user_name = cmd.args[0].clone();
                let result = self.delete_user(user_name);
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::GRANT => {
                self.check_auth(&session, Permissions::USER_ADMIN)?;
                // also check for other permissions here!

                let username = cmd.args[0].clone();
                let permissions = u8::from_str(&cmd.args[1]).unwrap();

                let permissions_to_set = Permissions::from_u8(permissions.clone());
                for p in permissions_to_set {
                    self.check_permission(&session, p)?;
                }

                self.auth_manager.grant_permissions(username, permissions)?;

                Ok(("OK".to_string(), session))
            }
            CommandNames::REVOKE => {
                self.check_auth(&session, Permissions::USER_ADMIN)?;

                let username = cmd.args[0].clone();
                let permission = u8::from_str(&cmd.args[1]).unwrap();

                let permissions_to_revoke = Permissions::from_u8(permission.clone());
                for p in permissions_to_revoke {
                    self.check_permission(&session, p)?;
                }

                self.auth_manager.revoke_permission(username, permission)?;

                Ok(("OK".to_string(), session))
            }
        }
    }

    fn check_permission(&self, session: &Session, permission: Permissions) -> Result<(), String> {
        if !self
            .auth_manager
            .check_permission(session.username.clone(), permission)
        {
            return Err("User does not have permission".to_string());
        }
        Ok(())
    }

    fn check_auth(&self, session: &Session, permission: Permissions) -> Result<(), String> {
        if !session.is_authenticated {
            return Err("User not authenticated".to_string());
        }
        if !self.auth_manager.has_user(session.username.clone()) {
            return Err("User not authenticated".to_string());
        }
        self.check_permission(session, permission)?;
        Ok(())
    }

    fn set(&mut self, key: String, value: String, data_type: DataTypes) -> Result<String, String> {
        self.data.insert(key, DataValue::new(value, data_type)?);
        Ok("OK".to_string())
    }

    fn get(&self, key: String) -> Result<String, String> {
        match self.data.get(&key) {
            Some(value) => Ok(value.value.clone()),
            None => Err("Key not found".to_string()),
        }
    }

    fn del(&mut self, key: String) -> Result<String, String> {
        match self.data.remove(&key) {
            Some(_) => Ok("OK".to_string()),
            None => Err("Key not found".to_string()),
        }
    }

    fn auth(
        &self,
        user_name: String,
        password: String,
        session: Session,
    ) -> Result<Session, String> {
        self.auth_manager.login_user(user_name, password, session)
    }

    fn create_user(
        &mut self,
        user_name: String,
        password: String,
        permissions: u8,
    ) -> Result<String, String> {
        self.auth_manager
            .create_user(user_name, password, permissions)
    }

    fn delete_user(&mut self, user_name: String) -> Result<String, String> {
        self.auth_manager.delete_user(user_name)
    }
}
