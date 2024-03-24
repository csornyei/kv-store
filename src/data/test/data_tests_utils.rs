use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    config::Config,
    data::{data_manager::DataManager, Store},
    session::Session,
};

pub async fn create_data_manager() -> DataManager {
    let config = Config::new();
    let shared_config = Arc::new(Mutex::new(config));

    let store = Store::new(".".to_string());
    let shared_store = Arc::new(Mutex::new(store));

    DataManager::new(shared_store, shared_config).await.unwrap()
}

pub fn create_session() -> Session {
    Session::new().set_authenticated("admin")
}
