use std::str::FromStr;

use crate::{
    commands::Command,
    data::{data_tests_utils::*, DataTypes},
};

#[test]
fn test_command_set_validate_args() {
    let cmd = Command::from_str("SET key true BOOL").unwrap();

    let key = cmd.args[0].clone();

    assert_eq!(key, "key".to_string());

    let value = cmd.args[1].clone();

    assert_eq!(value, "true".to_string());

    let data_type = DataTypes::from_str(&cmd.args[2]).unwrap();

    assert_eq!(data_type, DataTypes::BOOL);
}

#[test]
fn test_command_set_simple_key() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key value").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "OK".to_string());
}

#[test]
fn test_command_set_int() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key 10 INT").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "OK".to_string());
}

#[test]
fn test_command_set_float() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key 10.5 FLOAT").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "OK".to_string());
}

#[test]
fn test_command_set_bool() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key true BOOL").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "OK".to_string());
}

#[test]
fn test_command_set_key_in_store() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("SET store:key value").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "OK".to_string());
}

#[test]
fn test_command_set_key_in_embedded_store() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("CREATE_STORE store:substore").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("SET store:substore:key value").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "OK".to_string());
}
