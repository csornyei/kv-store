use std::sync::Arc;

use argon2::{
    password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use tokio::sync::Mutex;

use crate::data::{DataTypes, Key, Store, StoreManager};

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub permissions: u8,
}

pub struct UserKeys {
    pub user_store_key: Key,
    pub username_key: Key,
    pub password_key: Key,
    pub permissions_key: Key,
}

impl User {
    pub fn new(username: String, password: String, permissions: u8) -> Result<User, Error> {
        let hash = User::hash_password(&password)?;
        Ok(User {
            username,
            password: hash,
            permissions,
        })
    }

    pub async fn from_store(username: &str, store: Arc<Mutex<Store>>) -> Result<User, String> {
        let user_key = Key::new(format!("_auth:users:{}", username));

        let mut store = store.lock().await;

        let user = store.get_store(user_key)?;

        let username = user.get(Key::new("username".to_string()))?;
        let password = user.get(Key::new("password".to_string()))?;
        let permissions = user.get(Key::new("permissions".to_string()))?;

        Ok(User {
            username,
            password,
            permissions: permissions.parse::<u8>().unwrap(),
        })
    }

    pub async fn create(
        username: &str,
        password: &str,
        permissions: u8,
        store: Arc<Mutex<Store>>,
    ) -> Result<User, String> {
        let user = match User::new(username.to_string(), password.to_string(), permissions) {
            Ok(user) => user,
            Err(_) => return Err("Error creating user".to_string()),
        };

        {
            let mut store = store.lock().await;

            let user_keys = user.get_user_keys();

            store.set_store(user_keys.user_store_key.clone())?;
        }

        user.save(store).await?;

        Ok(user)
    }

    pub async fn save(&self, store: Arc<Mutex<Store>>) -> Result<(), String> {
        let user_keys = self.get_user_keys();

        let mut store = store.lock().await;

        store.set(
            user_keys.username_key,
            self.username.clone(),
            DataTypes::STRING,
        )?;
        store.set(
            user_keys.password_key,
            self.password.clone(),
            DataTypes::STRING,
        )?;
        store.set(
            user_keys.permissions_key,
            self.permissions.to_string(),
            DataTypes::STRING,
        )?;

        Ok(())
    }

    pub fn to_string(&self) -> String {
        format!("User: {} Permissions: {}", self.username, self.permissions)
    }

    fn hash_password(password: &str) -> Result<String, Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut rand::thread_rng());
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hash)
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, Error> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&self.password)?;

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn grant_permission(&mut self, permission: u8) -> User {
        User {
            username: self.username.clone(),
            password: self.password.clone(),
            permissions: self.permissions | permission,
        }
    }

    pub fn revoke_permission(&mut self, permission: u8) -> User {
        User {
            username: self.username.clone(),
            password: self.password.clone(),
            permissions: self.permissions & !permission,
        }
    }

    pub fn update_permissions(&self, permissions: u8) -> User {
        User {
            username: self.username.clone(),
            password: self.password.clone(),
            permissions,
        }
    }

    pub fn get_user_keys(&self) -> UserKeys {
        UserKeys {
            user_store_key: Key::new(format!("_auth:users:{}", self.username)),
            username_key: Key::new(format!("_auth:users:{}:username", self.username)),
            password_key: Key::new(format!("_auth:users:{}:password", self.username)),
            permissions_key: Key::new(format!("_auth:users:{}:permissions", self.username)),
        }
    }
}
