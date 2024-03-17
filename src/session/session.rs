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

    pub fn update(&mut self, new_session: Session) {
        self.is_authenticated = new_session.is_authenticated;
        self.username = new_session.username;
        self.incomplete_command = new_session.incomplete_command;
    }

    pub fn set_authenticated(&mut self, username: &str) -> Session {
        self.is_authenticated = true;
        self.username = username.to_string();

        self.clone()
    }

    pub fn resume_incomplete_command(&mut self, command: &str) -> Session {
        self.incomplete_command = command.to_string();

        self.clone()
    }
}
