use regex::Regex;
use std::collections::HashMap;

use crate::{
    auth::{Permissions, User},
    session::Session,
};

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

    pub fn create_user(
        &mut self,
        username: String,
        password: String,
        permission: u8,
    ) -> Result<String, String> {
        Self::validate_password(&password)?;

        if self.users.contains_key(&username) {
            return Err("User already exists".to_string());
        }

        let new_user = match User::new(username.clone(), password.clone(), permission) {
            Ok(user) => user,
            Err(e) => return Err(e.to_string()),
        };

        self.users.insert(username.clone(), new_user);

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

    pub fn login_user(
        &self,
        username: String,
        password: String,
        mut session: Session,
    ) -> Result<Session, String> {
        Self::validate_password(&password)?;

        match self.users.get(&username) {
            None => return Err("Username or password is incorrect".to_string()),
            Some(user) => {
                if user.verify_password(&password).map_err(|e| e.to_string())? {
                    Ok(session.set_authenticated(&username))
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
                self.users
                    .insert(username.clone(), user.update_permissions(new_permissions));
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
                self.users
                    .insert(username.clone(), user.update_permissions(new_permissions));
                Ok("OK".to_string())
            }
        }
    }
}
