use crate::commands::{Command, CommandNames};
use crate::data::auth_manager::AuthManager;
use crate::session::Session;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DataManager {
    data: HashMap<String, String>,
    auth_manager: AuthManager,
}

impl DataManager {
    pub fn new(admin_username: String, admin_password: String) -> Result<DataManager, String> {
        let auth_manager = AuthManager::new(admin_username, admin_password)?;
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
                self.check_auth(&session)?;
                let key = cmd.args[0].clone();
                let value = cmd.args[1].clone();
                let result = self.set(key, value);
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::GET => {
                self.check_auth(&session)?;
                let key = cmd.args[0].clone();
                let result = self.get(key);
                match result {
                    Ok(value) => Ok((value, session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::DEL => {
                self.check_auth(&session)?;
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
                let result = self.auth(user_name, password);
                match result {
                    Ok(session) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::CREATE_USER => {
                self.check_auth(&session)?;
                let user_name = cmd.args[0].clone();
                let password = cmd.args[1].clone();
                let result = self.create_user(user_name, password);
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::DELETE_USER => {
                self.check_auth(&session)?;
                let user_name = cmd.args[0].clone();
                let result = self.delete_user(user_name);
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn check_auth(&self, session: &Session) -> Result<(), String> {
        if !session.is_authenticated {
            return Err("User not authenticated".to_string());
        }
        if !self.auth_manager.has_user(session.username.clone()) {
            return Err("User not authenticated".to_string());
        }
        Ok(())
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

    fn auth(&self, _user_name: String, password: String) -> Result<Session, String> {
        self.auth_manager.login_user(_user_name, password)
    }

    fn create_user(&mut self, user_name: String, password: String) -> Result<String, String> {
        self.auth_manager.create_user(user_name, password)
    }

    fn delete_user(&mut self, user_name: String) -> Result<String, String> {
        self.auth_manager.delete_user(user_name)
    }
}

#[cfg(test)]
mod data_manager_tests {
    use std::str::FromStr;

    use super::*;
    use crate::session::Session;

    fn create_data_manager() -> DataManager {
        DataManager::new("admin".to_string(), "Password4".to_string()).unwrap()
    }

    fn create_session() -> Session {
        Session {
            is_authenticated: true,
            username: "admin".to_string(),
        }
    }

    #[test]
    fn test_set() {
        let mut data = create_data_manager();
        assert_eq!(
            data.set("key".to_string(), "value".to_string()),
            Ok("OK".to_string())
        );
        assert_eq!(data.data.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_get() {
        let mut data = create_data_manager();
        data.data.insert("key".to_string(), "value".to_string());
        assert_eq!(data.get("key".to_string()), Ok("value".to_string()));
    }

    #[test]
    fn test_del() {
        let mut data = create_data_manager();
        data.data.insert("key".to_string(), "value".to_string());
        assert_eq!(data.del("key".to_string()), Ok("OK".to_string()));
        assert_eq!(data.data.get("key"), None);
    }

    #[test]
    fn test_auth() {
        let mut data = create_data_manager();

        let session = data
            .auth("user".to_string(), "Password4".to_string())
            .unwrap_err();

        assert_eq!(session, "Username or password is incorrect".to_string());

        assert_eq!(
            data.create_user("user".to_string(), "Password4".to_string()),
            Ok("OK".to_string())
        );

        let session = data
            .auth("user".to_string(), "Password4".to_string())
            .unwrap();

        assert_eq!(session.username, "user".to_string());
    }

    #[test]
    fn test_create_user() {
        let mut data = create_data_manager();
        assert_eq!(
            data.create_user("user".to_string(), "Password4".to_string()),
            Ok("OK".to_string())
        );

        let session = data
            .auth("user".to_string(), "Password4".to_string())
            .unwrap();

        assert_eq!(session.username, "user".to_string());
    }

    #[test]
    fn test_delete_user() {
        let mut data = create_data_manager();
        assert_eq!(
            data.create_user("user".to_string(), "Password4".to_string()),
            Ok("OK".to_string())
        );

        let session = data
            .auth("user".to_string(), "Password4".to_string())
            .unwrap();

        assert_eq!(session.username, "user");

        assert_eq!(data.delete_user("user".to_string()), Ok("OK".to_string()));

        let session = data
            .auth("user".to_string(), "Password4".to_string())
            .unwrap_err();

        assert_eq!(session, "Username or password is incorrect".to_string());
    }

    #[test]
    fn test_handle_command_set() {
        let mut data = create_data_manager();
        let cmd = Command::from_str("SET key value").unwrap();

        let result = data.handle_command(cmd, create_session());

        assert_eq!(result, Ok(("OK".to_string(), create_session())));
        assert_eq!(data.data.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_handle_command_get() {
        let mut data = create_data_manager();
        data.data.insert("key".to_string(), "value".to_string());
        let cmd = Command::from_str("GET key").unwrap();

        let result = data.handle_command(cmd, create_session());

        assert_eq!(result, Ok(("value".to_string(), create_session())));
    }

    #[test]
    fn test_handle_command_del() {
        let mut data = create_data_manager();
        data.data.insert("key".to_string(), "value".to_string());
        let cmd = Command::from_str("DEL key").unwrap();

        let result = data.handle_command(cmd, create_session());

        assert_eq!(result, Ok(("OK".to_string(), create_session())));
        assert_eq!(data.data.get("key"), None);
    }

    #[test]
    fn test_handle_command_auth() {
        let mut data = create_data_manager();
        let cmd = Command::from_str("AUTH user Password4").unwrap();

        let result = data.handle_command(cmd, create_session());

        assert_eq!(result, Err("Username or password is incorrect".to_string()));

        let cmd = Command::from_str("CREATE_USER user Password4").unwrap();

        let (result, _) = data.handle_command(cmd, create_session()).unwrap();

        assert_eq!(result, "OK".to_string());

        let cmd = Command::from_str("AUTH user Password4").unwrap();

        let result = data.handle_command(cmd, Session::new()).unwrap();

        assert_eq!(result.0, "OK".to_string());
        assert_eq!(result.1.username, "user".to_string());
    }

    #[test]
    fn test_handle_command_create_user() {
        let mut data = create_data_manager();
        let cmd = Command::from_str("CREATE_USER user Password4").unwrap();

        let result = data.handle_command(cmd, create_session());

        assert_eq!(result, Ok(("OK".to_string(), create_session())));

        let cmd = Command::from_str("AUTH user Password4").unwrap();

        let (result, session) = data.handle_command(cmd, create_session()).unwrap();

        assert_eq!(result, "OK".to_string());
        assert_eq!(session.username, "user".to_string());
    }

    #[test]
    fn test_handle_command_delete_user() {
        let mut data = create_data_manager();

        let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
        let result = data.handle_command(cmd, create_session()).unwrap();

        assert_eq!(result, ("OK".to_string(), create_session()));

        let cmd = Command::from_str("DELETE_USER user").unwrap();

        let result = data.handle_command(cmd, create_session()).unwrap();
        assert_eq!(result, ("OK".to_string(), create_session()));
        let cmd = Command::from_str("AUTH user Password4").unwrap();

        let result = data.handle_command(cmd, create_session()).unwrap_err();

        assert_eq!(result, "Username or password is incorrect".to_string());
    }

    #[test]
    fn test_handle_command_flow() {
        let mut data = create_data_manager();
        let cmd = Command::from_str("GET key").unwrap();
        let result = data.handle_command(cmd, create_session()).unwrap_err();
        assert_eq!(result, "Key not found".to_string());

        let cmd = Command::from_str("SET key value").unwrap();
        let result = data.handle_command(cmd, create_session()).unwrap();
        assert_eq!(result, ("OK".to_string(), create_session()));

        let cmd = Command::from_str("GET key").unwrap();
        let result = data.handle_command(cmd, create_session()).unwrap();
        assert_eq!(result, ("value".to_string(), create_session()));

        let cmd = Command::from_str("DEL key").unwrap();
        let result = data.handle_command(cmd, create_session()).unwrap();
        assert_eq!(result, ("OK".to_string(), create_session()));

        let cmd = Command::from_str("GET key").unwrap();
        let result = data.handle_command(cmd, create_session()).unwrap_err();
        assert_eq!(result, "Key not found".to_string());
    }

    #[test]
    fn test_check_auth() {
        let mut data = create_data_manager();
        let session = Session::new();
        let result = data.check_auth(&session).unwrap_err();
        assert_eq!(result, "User not authenticated".to_string());

        // Test all commands that require authentication
        let cmd = Command::from_str("SET key value").unwrap();
        let result = data.handle_command(cmd, session.clone()).unwrap_err();
        assert_eq!(result, "User not authenticated".to_string());

        let cmd = Command::from_str("GET key").unwrap();
        let result = data.handle_command(cmd, session.clone()).unwrap_err();
        assert_eq!(result, "User not authenticated".to_string());

        let cmd = Command::from_str("DEL key").unwrap();
        let result = data.handle_command(cmd, session.clone()).unwrap_err();
        assert_eq!(result, "User not authenticated".to_string());

        let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
        let result = data.handle_command(cmd, session.clone()).unwrap_err();
        assert_eq!(result, "User not authenticated".to_string());

        let cmd = Command::from_str("DELETE_USER user").unwrap();
        let result = data.handle_command(cmd, session.clone()).unwrap_err();
        assert_eq!(result, "User not authenticated".to_string());
    }

    #[test]
    fn test_check_auth_deleted_user() {
        let mut data = create_data_manager();
        let admin_session = create_session();

        let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
        let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
        assert_eq!(result, "OK".to_string());

        let cmd = Command::from_str("AUTH user Password4").unwrap();
        let (result, user_session) = data.handle_command(cmd, Session::new()).unwrap();
        assert_eq!(result, "OK".to_string());

        let cmd = Command::from_str("SET key value").unwrap();
        let (result, _) = data.handle_command(cmd, user_session.clone()).unwrap();
        assert_eq!(result, "OK".to_string());

        let cmd = Command::from_str("GET key").unwrap();
        let (result, _) = data.handle_command(cmd, user_session.clone()).unwrap();
        assert_eq!(result, "value".to_string());

        let cmd = Command::from_str("DELETE_USER user").unwrap();
        let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
        assert_eq!(result, "OK".to_string());

        let cmd = Command::from_str("GET key").unwrap();
        let result = data.handle_command(cmd, user_session.clone()).unwrap_err();
        assert_eq!(result, "User not authenticated".to_string());

        let cmd = Command::from_str("GET key").unwrap();
        let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
        assert_eq!(result, "value".to_string());
    }
}
