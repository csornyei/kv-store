#[derive(Debug, PartialEq, Clone)]
pub struct Session {
    pub is_authenticated: bool,
    pub username: String,
}

impl Session {
    pub fn new() -> Self {
        Self {
            is_authenticated: false,
            username: "".to_string(),
        }
    }
}
