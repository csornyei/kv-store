pub struct Key {
    pub store: Option<String>,
    pub path: Option<String>,
    pub key: Option<String>,
}

impl Key {
    pub fn new(key: String) -> Key {
        if key == "." {
            return Key {
                store: Some(".".to_string()),
                path: None,
                key: None,
            };
        }
        let mut key_parts: Vec<&str> = key.split(":").collect();
        if key_parts.len() == 1 {
            let key = key_parts[0].to_string();
            Key {
                store: None,
                path: None,
                key: Some(key),
            }
        } else if key_parts.len() == 2 {
            let store = key_parts[0].to_string();
            let key = key_parts[1].to_string();
            Key {
                store: Some(store),
                path: None,
                key: Some(key),
            }
        } else {
            let store = key_parts.remove(0).to_string();
            let key = key_parts.pop().unwrap().to_string();
            let path = key_parts.join(":");

            Key {
                store: Some(store),
                path: Some(path),
                key: Some(key),
            }
        }
    }

    pub fn to_str(&self) -> String {
        let mut key = String::new();
        if self.store.is_some() {
            key.push_str(&self.store.clone().unwrap());
        }
        if self.path.is_some() {
            if key.len() > 0 {
                key.push_str(":");
            }
            key.push_str(&self.path.clone().unwrap());
        }
        if self.key.is_some() {
            if key.len() > 0 {
                key.push_str(":");
            }
            key.push_str(&self.key.clone().unwrap());
        }
        key
    }

    pub fn get_next_key(mut self) -> Key {
        self.store = None;

        let key_str = self.to_str();

        Key::new(key_str)
    }

    pub fn valid_get_key(&self) -> bool {
        if self.key.is_none() {
            return false;
        }
        true
    }
}
