use regex::Regex;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{
    auth::{Permissions, User},
    data::{Key, Store, StoreManager},
    session::Session,
};

pub struct AuthManager {
    store_access: Arc<Mutex<Store>>,
}

impl AuthManager {
    pub async fn new(store_access: Arc<Mutex<Store>>) -> Result<AuthManager, String> {
        let mut auth_manager = AuthManager {
            store_access: store_access.clone(),
        };

        auth_manager.setup_auth_store().await?;

        Ok(auth_manager)
    }

    async fn setup_auth_store(&mut self) -> Result<(), String> {
        let mut store = self.store_access.lock().await;

        match store.get_store(Key::new("_auth".to_string())) {
            Ok(_) => match store.get_store(Key::new("_auth:users".to_string())) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    store.set_store(Key::new("_auth:users".to_string()))?;
                    Ok(())
                }
            },
            Err(_) => {
                store.set_store(Key::new("_auth".to_string()))?;
                store.set_store(Key::new("_auth:users".to_string()))?;
                Ok(())
            }
        }
    }

    fn validate_password(password: &str) -> Result<(), String> {
        if password.len() < 8 {
            return Err("Password too short!".to_string());
        }

        let lowercase_re = Regex::new(r"[a-z]").unwrap();

        if !lowercase_re.is_match(password) {
            return Err("Password must contain a lowercase letter!".to_string());
        }

        let uppercase_re = Regex::new(r"[A-Z]").unwrap();

        if !uppercase_re.is_match(password) {
            return Err("Password must contain an uppercase letter!".to_string());
        }

        let digit_re = Regex::new(r"\d").unwrap();

        if !digit_re.is_match(password) {
            return Err("Password must contain a digit!".to_string());
        }

        Ok(())
    }

    pub async fn create_user(
        &mut self,
        username: String,
        password: String,
        permission: u8,
    ) -> Result<String, String> {
        Self::validate_password(&password)?;

        User::create(
            &username,
            &password,
            permission,
            Arc::clone(&self.store_access),
        )
        .await?;

        Ok("OK".to_string())
    }

    pub async fn delete_user(&mut self, username: String) -> Result<String, String> {
        let user = User::from_store(&username, Arc::clone(&self.store_access)).await?;

        let user_keys = user.get_user_keys();

        let mut store = self.store_access.lock().await;

        store.del(user_keys.user_store_key)
    }

    pub async fn login_user(
        &self,
        username: String,
        password: String,
        mut session: Session,
    ) -> Result<Session, String> {
        Self::validate_password(&password)?;

        let user = match User::from_store(&username, Arc::clone(&self.store_access)).await {
            Ok(user) => user,
            Err(_) => return Err("Username or password is incorrect".to_string()),
        };

        if user.verify_password(&password).map_err(|e| e.to_string())? {
            Ok(session.set_authenticated(&username))
        } else {
            Err("Username or password is incorrect".to_string())
        }
    }

    pub async fn has_user(&self, username: String) -> bool {
        User::from_store(&username, Arc::clone(&self.store_access))
            .await
            .is_ok()
    }

    pub async fn check_permission(&self, username: String, permission: Permissions) -> bool {
        let user = match User::from_store(&username, Arc::clone(&self.store_access)).await {
            Ok(user) => user,
            Err(_) => return false,
        };

        user.permissions & (permission as u8) != 0
    }

    pub async fn get_user(&self, username: String) -> Option<User> {
        match User::from_store(&username, Arc::clone(&self.store_access)).await {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    pub async fn grant_permissions(
        &mut self,
        username: String,
        permission: u8,
    ) -> Result<String, String> {
        let mut user = User::from_store(&username, Arc::clone(&self.store_access)).await?;

        let user = user.grant_permission(permission);

        user.save(Arc::clone(&self.store_access)).await?;

        Ok("OK".to_string())
    }

    pub async fn revoke_permission(
        &mut self,
        username: String,
        permission: u8,
    ) -> Result<String, String> {
        let mut user = User::from_store(&username, Arc::clone(&self.store_access)).await?;

        let user = user.revoke_permission(permission);

        user.save(Arc::clone(&self.store_access)).await?;

        Ok("OK".to_string())
    }
}
