use super::data_manager::*;
use crate::session::Session;

pub fn create_data_manager() -> DataManager {
    DataManager::new("admin".to_string(), "Password4".to_string()).unwrap()
}

pub fn create_session() -> Session {
    Session::new().set_authenticated("admin")
}
