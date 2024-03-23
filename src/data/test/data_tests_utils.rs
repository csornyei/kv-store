use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    data::{data_manager::DataManager, Store},
    session::Session,
};

pub fn create_data_manager() -> DataManager {
    DataManager::new(Arc::new(Mutex::new(Store::new(".".to_string())))).unwrap()
}

pub fn create_session() -> Session {
    Session::new().set_authenticated("admin")
}
