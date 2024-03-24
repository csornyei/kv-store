use argon2::{
    password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::data::Key;

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