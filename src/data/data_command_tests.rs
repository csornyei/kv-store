use std::str::FromStr;

use super::data_tests_utils::{create_data_manager, create_session};
use crate::commands::Command;

#[test]
fn test_handle_command_set() {
    let mut data = create_data_manager();
    let cmd = Command::from_str("SET key value").unwrap();

    let result = data.handle_command(cmd, create_session());

    assert_eq!(result, Ok(("OK".to_string(), create_session())));

    let cmd = Command::from_str("GET key").unwrap();

    let (result, _) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "value".to_string());
}

#[test]
fn test_handle_command_get() {
    let mut data = create_data_manager();
    let cmd = Command::from_str("SET key value").unwrap();

    data.handle_command(cmd, create_session()).unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let result = data.handle_command(cmd, create_session());

    assert_eq!(result, Ok(("value".to_string(), create_session())));
}

#[test]
fn test_handle_command_del() {
    let mut data = create_data_manager();

    let cmd = Command::from_str("SET key value").unwrap();

    data.handle_command(cmd, create_session()).unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let (result, _) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "value".to_string());

    let cmd = Command::from_str("DEL key").unwrap();

    let result = data.handle_command(cmd, create_session());

    assert_eq!(result, Ok(("OK".to_string(), create_session())));

    let cmd = Command::from_str("GET key").unwrap();

    let result = data.handle_command(cmd, create_session()).unwrap_err();

    assert_eq!(result, "Key not found".to_string());
}
