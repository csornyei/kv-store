use crate::session::Session;
use argon2::{
    password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use regex::Regex;
use std::collections::HashMap;

// Permissions bit mask:
// 0b00000001 - SET
// 0b00000010 - GET
// 0b00000100 - DEL
// 0b00001000 - CREATE_USER & DELETE_USER
// To GRANT permission user needs 0b00001000 & appropriate permission:
// 0b00001000 | 0b00000001 = 0b00001001
// 0b00001000 | 0b00000010 = 0b00001010
// 0b00001000 | 0b00000100 = 0b00001100

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Permissions {
    NONE = 0,
    SET = 1 << 0,
    GET = 1 << 1,
    DEL = 1 << 2,
    USER_ADMIN = 1 << 3,
}

impl Permissions {
    pub fn from_u8(value: u8) -> Vec<Permissions> {
        let mut permissions = Vec::new();

        if value & (Permissions::SET as u8) != 0 {
            permissions.push(Permissions::SET);
        }

        if value & (Permissions::GET as u8) != 0 {
            permissions.push(Permissions::GET);
        }

        if value & (Permissions::DEL as u8) != 0 {
            permissions.push(Permissions::DEL);
        }

        if value & (Permissions::USER_ADMIN as u8) != 0 {
            permissions.push(Permissions::USER_ADMIN);
        }

        permissions
    }
}

#[derive(Debug)]
pub struct User {
    username: String,
    password: String,
    permissions: u8,
}

impl User {
    pub fn new(username: String, password: String, permissions: u8) -> User {
        User {
            username,
            password,
            permissions,
        }
    }
    pub fn to_string(&self) -> String {
        format!("User: {} Permissions: {}", self.username, self.permissions)
    }
}

#[derive(Debug)]
pub struct AuthManager {
    users: HashMap<String, User>, // make it persistent and encrypted
}

impl AuthManager {
    pub fn new(
        admin_username: String,
        admin_password: String,
        permission: u8,
    ) -> Result<AuthManager, String> {
        let mut auth_manager = AuthManager {
            users: HashMap::new(),
        };

        auth_manager.create_user(admin_username, admin_password, permission)?;

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

    pub fn create_user(
        &mut self,
        username: String,
        password: String,
        permission: u8,
    ) -> Result<String, String> {
        Self::validate_password(&password)?;

        let hash = Self::hash_password(&password).map_err(|e| e.to_string())?;

        self.users.insert(
            username.clone(),
            User::new(username.clone(), hash, permission),
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

    pub fn check_permission(&self, username: String, permission: Permissions) -> bool {
        match self.users.get(&username) {
            None => false,
            Some(user) => user.permissions & (permission as u8) != 0,
        }
    }

    pub fn get_user(&self, username: String) -> Option<&User> {
        self.users.get(&username)
    }

    pub fn grant_permissions(
        &mut self,
        username: String,
        permission: u8,
    ) -> Result<String, String> {
        match self.users.get(&username) {
            None => return Err("User does not exist".to_string()),
            Some(user) => {
                let new_permissions = user.permissions | permission;
                self.users.insert(
                    username.clone(),
                    User::new(username.clone(), user.password.clone(), new_permissions),
                );
                Ok("OK".to_string())
            }
        }
    }

    pub fn revoke_permission(
        &mut self,
        username: String,
        permission: u8,
    ) -> Result<String, String> {
        match self.users.get(&username) {
            None => return Err("User does not exist".to_string()),
            Some(user) => {
                let new_permissions = user.permissions & !permission;
                self.users.insert(
                    username.clone(),
                    User::new(username, user.password.clone(), new_permissions),
                );
                Ok("OK".to_string())
            }
        }
    }
}
