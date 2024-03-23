use std::str::FromStr;

use super::commands::Command;
use super::CommandNames;

#[test]
fn test_validate_set_args() {
    match Command::from_str("SET") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    match Command::from_str("SET key") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    let command = Command::from_str("SET key value").unwrap();

    assert_eq!(command.name, CommandNames::SET);
    assert_eq!(command.args, vec!["key", "value", "STRING"]);

    match Command::from_str("SET key value1 not_a_datatype") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid data type"),
    }

    let command = Command::from_str("SET key value1 STRING").unwrap();

    assert_eq!(command.name, CommandNames::SET);
    assert_eq!(command.args, vec!["key", "value1", "STRING"]);

    match Command::from_str("SET key value1 INT") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid data type"),
    }

    match Command::from_str("SET key value1 FLOAT") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid data type"),
    }

    match Command::from_str("SET key value1 BOOL") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid data type"),
    }

    match Command::from_str("SET key value1 STORE") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(
            e.to_string(),
            "Invalid data type. To create STORE use CREATE_STORE command"
        ),
    }
}

#[test]
fn test_validate_get_args() {
    let command = Command::from_str("GET key").unwrap();

    assert_eq!(command.name, CommandNames::GET);
    assert_eq!(command.args, vec!["key"]);

    match Command::from_str("GET") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    match Command::from_str("GET key1 key2") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }
}

#[test]
fn test_validate_del_args() {
    let command = Command::from_str("DEL key").unwrap();

    assert_eq!(command.name, CommandNames::DEL);
    assert_eq!(command.args, vec!["key"]);

    match Command::from_str("DEL") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    match Command::from_str("DEL key1 key2") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }
}

#[test]
fn test_validate_auth_args() {
    let command = Command::from_str("AUTH username password").unwrap();

    assert_eq!(command.name, CommandNames::AUTH);
    assert_eq!(command.args, vec!["username", "password"]);

    match Command::from_str("AUTH") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    match Command::from_str("AUTH username password1 password2") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }
}

#[test]
fn test_validate_get_user_args() {
    let command = Command::from_str("GET_USER username").unwrap();

    assert_eq!(command.name, CommandNames::GET_USER);
    assert_eq!(command.args, vec!["username"]);

    match Command::from_str("GET_USER") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    match Command::from_str("GET_USER username1 username2") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }
}

#[test]
fn test_validate_create_user_args() {
    let command = Command::from_str("CREATE_USER username password").unwrap();

    assert_eq!(command.name, CommandNames::CREATE_USER);
    assert_eq!(command.args, vec!["username", "password", "0"]);

    match Command::from_str("CREATE_USER") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    let command =
        Command::from_str("CREATE_USER username1 password1 USER_ADMIN SET GET DEL").unwrap();

    assert_eq!(command.name, CommandNames::CREATE_USER);
    assert_eq!(command.args, vec!["username1", "password1", "15"]);
}

#[test]
fn test_validate_delete_user_args() {
    let command = Command::from_str("DELETE_USER username").unwrap();

    assert_eq!(command.name, CommandNames::DELETE_USER);
    assert_eq!(command.args, vec!["username"]);

    match Command::from_str("DELETE_USER") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    match Command::from_str("DELETE_USER username1 username2") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }
}

#[test]
fn test_validate_grant_args() {
    let command = Command::from_str("GRANT username SET").unwrap();

    assert_eq!(command.name, CommandNames::GRANT);
    assert_eq!(command.args, vec!["username", "1"]);

    match Command::from_str("GRANT") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    let command = Command::from_str("GRANT username1 SET GET DEL USER_ADMIN").unwrap();

    assert_eq!(command.name, CommandNames::GRANT);
    assert_eq!(command.args, vec!["username1", "15"]);
}

#[test]
fn test_validate_revoke_args() {
    let command = Command::from_str("REVOKE username SET").unwrap();

    assert_eq!(command.name, CommandNames::REVOKE);
    assert_eq!(command.args, vec!["username", "1"]);

    match Command::from_str("REVOKE") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    let command = Command::from_str("REVOKE username1 SET GET DEL USER_ADMIN").unwrap();

    assert_eq!(command.name, CommandNames::REVOKE);
    assert_eq!(command.args, vec!["username1", "15"]);
}

#[test]
fn test_validate_create_store_args() {
    let command = Command::from_str("CREATE_STORE store").unwrap();

    assert_eq!(command.name, CommandNames::CREATE_STORE);
    assert_eq!(command.args, vec!["store"]);

    match Command::from_str("CREATE_STORE") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    match Command::from_str("CREATE_STORE store1 store2") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Invalid number of arguments"),
    }

    match Command::from_str("CREATE_STORE .") {
        Ok(_) => panic!("Expected error"),
        Err(e) => assert_eq!(e.to_string(), "Forbidden store name! ."),
    }

    let command = Command::from_str("CREATE_STORE store1:store2").unwrap();

    assert_eq!(command.name, CommandNames::CREATE_STORE);
    assert_eq!(command.args, vec!["store1:store2"]);
}
