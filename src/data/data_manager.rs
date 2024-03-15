use crate::commands::{Command, CommandNames};
use crate::data::auth_manager::{hash_password, verify_password};
use std::collections::HashMap;

use super::validate_password;

#[derive(Debug)]
pub struct DataManager {
    data: HashMap<String, String>,
    users: HashMap<String, String>,
}

impl DataManager {
    pub fn new() -> DataManager {
        DataManager {
            data: HashMap::new(),
            users: HashMap::new(),
        }
    }

    pub fn handle_command(&mut self, cmd: Command) -> Result<String, String> {
        match cmd.name {
            CommandNames::SET => {
                let key = cmd.args[0].clone();
                let value = cmd.args[1].clone();
                return self.set(key, value);
            }
            CommandNames::GET => {
                let key = cmd.args[0].clone();
                return self.get(key);
            }
            CommandNames::DEL => {
                let key = cmd.args[0].clone();
                return self.del(key);
            }
            CommandNames::AUTH => {
                let user_name = cmd.args[0].clone();
                let password = cmd.args[1].clone();
                return self.auth(user_name, password);
            }
            CommandNames::CREATE_USER => {
                let user_name = cmd.args[0].clone();
                let password = cmd.args[1].clone();
                return self.create_user(user_name, password);
            }
            CommandNames::DELETE_USER => {
                let user_name = cmd.args[0].clone();
                return self.delete_user(user_name);
            }
        }
    }

    fn set(&mut self, key: String, value: String) -> Result<String, String> {
        self.data.insert(key, value);
        Ok("OK".to_string())
    }

    fn get(&self, key: String) -> Result<String, String> {
        match self.data.get(&key) {
            Some(value) => Ok(value.clone()),
            None => Err("Key not found".to_string()),
        }
    }

    fn del(&mut self, key: String) -> Result<String, String> {
        match self.data.remove(&key) {
            Some(_) => Ok("OK".to_string()),
            None => Err("Key not found".to_string()),
        }
    }

    fn auth(&self, _user_name: String, password: String) -> Result<String, String> {
        validate_password(&password)?;

        let users_pw = self.users.get(&_user_name).ok_or("User not found")?;

        if verify_password(users_pw, &password).map_err(|e| e.to_string())? {
            Ok("OK".to_string())
        } else {
            Err("Invalid password".to_string())
        }
    }

    fn create_user(&mut self, user_name: String, password: String) -> Result<String, String> {
        validate_password(&password)?;

        let hash = hash_password(&password).map_err(|e| e.to_string())?;

        self.users.insert(user_name, hash);

        Ok("OK".to_string())
    }

    fn delete_user(&mut self, user_name: String) -> Result<String, String> {
        if self.users.contains_key(&user_name) {
            self.users.remove(&user_name);
            Ok("OK".to_string())
        } else {
            Err("User not found".to_string())
        }
    }
}

#[cfg(test)]
mod data_manager_tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_set() {
        let mut data = DataManager::new();
        assert_eq!(
            data.set("key".to_string(), "value".to_string()),
            Ok("OK".to_string())
        );
        assert_eq!(data.data.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_get() {
        let mut data = DataManager::new();
        data.data.insert("key".to_string(), "value".to_string());
        assert_eq!(data.get("key".to_string()), Ok("value".to_string()));
    }

    #[test]
    fn test_del() {
        let mut data = DataManager::new();
        data.data.insert("key".to_string(), "value".to_string());
        assert_eq!(data.del("key".to_string()), Ok("OK".to_string()));
        assert_eq!(data.data.get("key"), None);
    }

    #[test]
    fn test_handle_command_set() {
        let mut data = DataManager::new();
        let cmd = Command::from_str("SET key value").unwrap();
        assert_eq!(data.handle_command(cmd), Ok("OK".to_string()));
        assert_eq!(data.data.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_handle_command_get() {
        let mut data = DataManager::new();
        data.data.insert("key".to_string(), "value".to_string());
        let cmd = Command::from_str("GET key").unwrap();
        assert_eq!(data.handle_command(cmd), Ok("value".to_string()));
    }

    #[test]
    fn test_handle_command_del() {
        let mut data = DataManager::new();
        data.data.insert("key".to_string(), "value".to_string());
        let cmd = Command::from_str("DEL key").unwrap();
        assert_eq!(data.handle_command(cmd), Ok("OK".to_string()));
        assert_eq!(data.data.get("key"), None);
    }

    #[test]
    fn test_handle_command_flow() {
        let mut data = DataManager::new();
        let cmd = Command::from_str("GET key").unwrap();
        assert_eq!(data.handle_command(cmd), Err("Key not found".to_string()));
        let cmd = Command::from_str("SET key value").unwrap();
        assert_eq!(data.handle_command(cmd), Ok("OK".to_string()));
        let cmd = Command::from_str("GET key").unwrap();
        assert_eq!(data.handle_command(cmd), Ok("value".to_string()));
        let cmd = Command::from_str("DEL key").unwrap();
        assert_eq!(data.handle_command(cmd), Ok("OK".to_string()));
        let cmd = Command::from_str("GET key").unwrap();
        assert_eq!(data.handle_command(cmd), Err("Key not found".to_string()));
    }
}
