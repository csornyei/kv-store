use std::str::FromStr;

use crate::{commands::Command, data::data_tests_utils::*};

#[test]
fn test_command_get_simple_key() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key value").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "value".to_string());
}

#[test]
fn test_command_get_key_in_store() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("SET store:key value").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("GET store:key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "value".to_string());
}

#[test]
fn test_command_get_key_in_embedded_store() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("CREATE_STORE store:embedded").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("SET store:embedded:key value").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("GET store:embedded:key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "value".to_string());
}
