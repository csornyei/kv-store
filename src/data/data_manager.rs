use crate::commands::{Command, CommandNames};
use crate::data::auth_manager::{AuthManager, Permissions};
use crate::session::Session;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct DataManager {
    data: HashMap<String, String>,
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
                let result = self.set(key, value);
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
