use crate::session::Session;
use argon2::{
    password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct User {
    _username: String,
    password: String,
}

#[derive(Debug)]
pub struct AuthManager {
    users: HashMap<String, User>, // make it persistent and encrypted
}

impl AuthManager {
    pub fn new(admin_username: String, admin_password: String) -> Result<AuthManager, String> {
        let mut auth_manager = AuthManager {
            users: HashMap::new(),
        };

        auth_manager.create_user(admin_username, admin_password)?;

        Ok(auth_manager)
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

    fn hash_password(password: &str) -> Result<String, Error> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut rand::thread_rng());
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        Ok(hash)
    }

    fn verify_password(hash: &str, password: &str) -> Result<bool, Error> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(hash)?;

        match argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub fn create_user(&mut self, username: String, password: String) -> Result<String, String> {
        Self::validate_password(&password)?;

        let hash = Self::hash_password(&password).map_err(|e| e.to_string())?;

        self.users.insert(
            username.clone(),
            User {
                _username: username.clone(),
                password: hash,
            },
        );

        Ok("OK".to_string())
    }

    pub fn delete_user(&mut self, username: String) -> Result<String, String> {
        match self.users.get(&username) {
            None => return Err("User does not exist".to_string()),
            Some(_) => {
                self.users.remove(&username);
                Ok("OK".to_string())
            }
        }
    }

    pub fn login_user(&self, username: String, password: String) -> Result<Session, String> {
        Self::validate_password(&password)?;

        match self.users.get(&username) {
            None => return Err("Username or password is incorrect".to_string()),
            Some(user) => {
                if Self::verify_password(&user.password, &password).map_err(|e| e.to_string())? {
                    Ok(Session {
                        is_authenticated: true,
                        username: username,
                    })
                } else {
                    Err("Username or password is incorrect".to_string())
                }
            }
        }
    }

    pub fn has_user(&self, username: String) -> bool {
        self.users.contains_key(&username)
    }
}
