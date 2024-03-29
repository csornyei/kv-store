use tokio::sync::Mutex;

use super::{
    data_type::DataTypes,
    key::Key,
    store::{Store, StoreManager},
};
use crate::{
    auth::{AuthManager, Permissions},
    commands::{Command, CommandNames},
    config::{Config, PersistenceConfig},
    persistence::{Persistence, PersistenceType},
    session::Session,
};
use std::{str::FromStr, sync::Arc};

pub struct DataManager {
    pub data: Arc<Mutex<Store>>,
    auth_manager: AuthManager,
    pub persistence: PersistenceConfig,
}

impl DataManager {
    pub async fn new(
        data: Arc<Mutex<Store>>,
        config: Arc<Mutex<Config>>,
    ) -> Result<DataManager, String> {
        let mut auth_manager = AuthManager::new(Arc::clone(&data)).await.unwrap();

        let (admin_username, admin_password) = config.lock().await.get_admin_user();

        match auth_manager
            .create_user(admin_username, admin_password, 255)
            .await
        {
            Ok(_) => {}
            Err(e) => println!("Error creating admin user: {}", e),
        }

        let persistence = config.lock().await.persistence.clone();
        Ok(DataManager {
            data: data.clone(),
            auth_manager,
            persistence,
        })
    }

    pub async fn save_to_file(&self) -> Result<(), String> {
        match self.persistence.get_json_file() {
            Some(persistence) => {
                let data = &self.data.lock().await;
                match persistence.save_store(data) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            }
            None => Ok(()),
        }
    }

    pub async fn handle_command(
        &mut self,
        cmd: Command,
        session: Session,
    ) -> Result<(String, Session), String> {
        match cmd.name {
            CommandNames::SET => {
                self.check_auth(&session, Permissions::SET).await?;
                let key = Key::new(cmd.args[0].clone());
                let value = cmd.args[1].clone();
                let data_type = DataTypes::from_str(&cmd.args[2])?;
                let result = self.set(key, value, data_type).await;
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::GET => {
                self.check_auth(&session, Permissions::GET).await?;
                let key = Key::new(cmd.args[0].clone());
                let result = self.get(key).await;
                match result {
                    Ok(value) => Ok((value, session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::DEL => {
                self.check_auth(&session, Permissions::DEL).await?;
                let key = Key::new(cmd.args[0].clone());
                let result = self.del(key).await;
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::AUTH => {
                let user_name = cmd.args[0].clone();
                let password = cmd.args[1].clone();
                let result = self.auth(user_name, password, session).await;
                match result {
                    Ok(session) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::GET_USER => {
                self.check_auth(&session, Permissions::USER_ADMIN).await?;

                let username = cmd.args[0].clone();
                match self.auth_manager.get_user(username).await {
                    Some(user) => Ok((user.to_string(), session)),
                    None => Err("User not found".to_string()),
                }
            }
            CommandNames::CREATE_USER => {
                self.check_auth(&session, Permissions::USER_ADMIN).await?;
                let user_name = cmd.args[0].clone();
                let password = cmd.args[1].clone();
                let permissions = u8::from_str(&cmd.args[2]).unwrap();

                let permissions_to_set = Permissions::from_u8(permissions.clone());
                for p in permissions_to_set {
                    self.check_permission(&session, p).await?;
                }

                let result = self.create_user(user_name, password, permissions).await;
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::DELETE_USER => {
                self.check_auth(&session, Permissions::USER_ADMIN).await?;
                let user_name = cmd.args[0].clone();
                let result = self.delete_user(user_name).await;
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::GRANT => {
                self.check_auth(&session, Permissions::USER_ADMIN).await?;
                // also check for other permissions here!

                let username = cmd.args[0].clone();
                let permissions = u8::from_str(&cmd.args[1]).unwrap();

                let permissions_to_set = Permissions::from_u8(permissions.clone());
                for p in permissions_to_set {
                    self.check_permission(&session, p).await?;
                }

                self.auth_manager
                    .grant_permissions(username, permissions)
                    .await?;

                Ok(("OK".to_string(), session))
            }
            CommandNames::REVOKE => {
                self.check_auth(&session, Permissions::USER_ADMIN).await?;

                let username = cmd.args[0].clone();
                let permission = u8::from_str(&cmd.args[1]).unwrap();

                let permissions_to_revoke = Permissions::from_u8(permission.clone());
                for p in permissions_to_revoke {
                    self.check_permission(&session, p).await?;
                }

                self.auth_manager
                    .revoke_permission(username, permission)
                    .await?;

                Ok(("OK".to_string(), session))
            }
            CommandNames::CREATE_STORE => {
                self.check_auth(&session, Permissions::SET).await?;
                let store_name = Key::new(cmd.args[0].clone());

                let result = self.create_store(store_name).await;
                match result {
                    Ok(_) => Ok(("OK".to_string(), session)),
                    Err(e) => Err(e),
                }
            }
            CommandNames::LIST_KEYS => {
                self.check_auth(&session, Permissions::GET).await?;

                let key = cmd.args[0].clone();

                if key == "." {
                    let data = self.data.lock().await;
                    return Ok((data.list_keys()?, session));
                } else {
                    let mut data = self.data.lock().await;
                    let store = data.get_store(Key::new(key.clone()));
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

    async fn check_permission(
        &self,
        session: &Session,
        permission: Permissions,
    ) -> Result<(), String> {
        if !self
            .auth_manager
            .check_permission(session.username.clone(), permission)
            .await
        {
            return Err("User does not have permission".to_string());
        }
        Ok(())
    }

    async fn check_auth(&self, session: &Session, permission: Permissions) -> Result<(), String> {
        if !session.is_authenticated {
            return Err("User not authenticated".to_string());
        }
        if !self.auth_manager.has_user(session.username.clone()).await {
            return Err("User not authenticated".to_string());
        }
        self.check_permission(session, permission).await?;
        Ok(())
    }

    async fn set(
        &mut self,
        key: Key,
        value: String,
        data_type: DataTypes,
    ) -> Result<String, String> {
        let mut data = self.data.lock().await;
        data.set(key, value, data_type)?;
        Ok("OK".to_string())
    }

    async fn get(&mut self, key: Key) -> Result<String, String> {
        let mut data = self.data.lock().await;
        data.get(key)
    }

    async fn del(&mut self, key: Key) -> Result<String, String> {
        let mut data = self.data.lock().await;
        match data.del(key) {
            Ok(_) => Ok("OK".to_string()),
            Err(_) => Err("Key not found".to_string()),
        }
    }

    async fn auth(
        &self,
        user_name: String,
        password: String,
        session: Session,
    ) -> Result<Session, String> {
        self.auth_manager
            .login_user(user_name, password, session)
            .await
    }

    async fn create_user(
        &mut self,
        user_name: String,
        password: String,
        permissions: u8,
    ) -> Result<String, String> {
        self.auth_manager
            .create_user(user_name, password, permissions)
            .await
    }

    async fn delete_user(&mut self, user_name: String) -> Result<String, String> {
        self.auth_manager.delete_user(user_name).await
    }

    async fn create_store(&mut self, store_name: Key) -> Result<String, String> {
        let mut data = self.data.lock().await;
        data.set_store(store_name)?;
        Ok("OK".to_string())
    }
}
