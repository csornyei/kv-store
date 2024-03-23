use std::fs;

use serde::{Deserialize, Serialize};

use crate::persistence::Persistence;

#[derive(Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            address: "127.0.0.1".to_string(),
            port: 4000,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AdminConfig {
    pub username: String,
    pub password: String,
}

impl Default for AdminConfig {
    fn default() -> Self {
        AdminConfig {
            username: "admin".to_string(),
            password: "Password4".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub persistence: Persistence,
    pub admin: AdminConfig,
}

impl Config {
    pub fn new() -> Self {
        Config {
            server: ServerConfig::default(),
            persistence: Persistence::new_in_memory(),
            admin: AdminConfig::default(),
        }
    }

    pub fn load(path: String) -> Self {
        let config_yaml = match fs::read_to_string(path.clone()) {
            Ok(content) => content,
            Err(_) => {
                let config = Config::new();
                config.save(path.clone());
                return config;
            }
        };

        serde_yaml::from_str(&config_yaml).unwrap()
    }

    pub fn save(&self, path: String) {
        let config_yaml = serde_yaml::to_string(&self).unwrap();

        fs::write(path, config_yaml).unwrap();
    }
}
