use super::{
    auth_manager::{AuthManager, Permissions},
    data_type::DataTypes,
    data_value::Data,
    key::Key,
    store::{Store, StoreManager},
};
use crate::session::Session;
use crate::{
    commands::{Command, CommandNames},
    persistence::Persistence,
};
use std::str::FromStr;

pub struct DataManager {
    data: Store,
    auth_manager: AuthManager,
    pub persistence: Persistence,
}

impl DataManager {
    pub fn new(
        admin_username: String,
        admin_password: String,
        persistence: Persistence,
    ) -> Result<DataManager, String> {
        let auth_manager = AuthManager::new(admin_username, admin_password, 255)?;
        let data = persistence.load_store()?;
        Ok(DataManager {
            data,
            auth_manager,
            persistence,
        })
    }

    pub fn save_to_file(&self) -> Result<(), String> {
        match self.persistence.save_store(&self.data) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn handle_command(
        &mut self,
        cmd: Command,
        session: Session,
    ) -> Result<(String, Session), String> {
        match cmd.name {
            CommandNames::SET => {
                self.check_auth(&session, Permissions::SET)?;
                let key = Key::new(cmd.args[0].clone());
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
                let key = Key::new(cmd.args[0].clone());
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
            CommandNames::CREATE_STORE => {
                self.check_auth(&session, Permissions::SET)?;
                let store_name = Key::new(cmd.args[0].clone());

                let result = self.create_store(store_name);
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::LIST_KEYS => {
                self.check_auth(&session, Permissions::GET)?;

                let key = cmd.args[0].clone();

                if key == "." {
                    return Ok((self.data.list_keys()?, session));
                } else {
                    let store = self.data.get_store(key.clone());
                    match store {
                        Ok(store) => {
                            return Ok((store.list_keys()?, session));
                        }
                        Err(_) => {
                            return Err("Invalid store".to_string());
                        }
                    }
                }
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

    fn set(&mut self, key: Key, value: String, data_type: DataTypes) -> Result<String, String> {
        self.data.set(key, value, data_type)?;
        Ok("OK".to_string())
    }

    fn get(&mut self, key: Key) -> Result<String, String> {
        self.data.get(key)
    }

    fn del(&mut self, key: String) -> Result<String, String> {
        match self.data.del(key) {
            Ok(_) => Ok("OK".to_string()),
            Err(_) => Err("Key not found".to_string()),
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

    fn create_store(&mut self, store_name: Key) -> Result<String, String> {
        self.data.set_store(store_name)?;
        Ok("OK".to_string())
    }
}
