#[derive(Debug, PartialEq, Clone)]
pub struct Session {
    pub is_authenticated: bool,
    pub username: String,
    pub incomplete_command: String,
}

impl Session {
    pub fn new() -> Self {
        Self {
            is_authenticated: false,
            username: "".to_string(),
            incomplete_command: "".to_string(),
        }
    }

    pub fn set_authenticated(&mut self, username: &str) -> Session {
        self.is_authenticated = true;
        self.username = username.to_string();

        self.clone()
    }
}
