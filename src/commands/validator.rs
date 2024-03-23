use super::CommandNames;
use crate::data::DataTypes;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

pub fn validate_args(name: &CommandNames, args: Vec<String>) -> Result<(), Error> {
    match name {
        CommandNames::SET => validate_set_args(args),
        CommandNames::GET => validate_get_args(args),
        CommandNames::DEL => validate_del_args(args),
        CommandNames::AUTH => validate_auth_args(args),
        CommandNames::GET_USER => validate_get_user_args(args),
        CommandNames::CREATE_USER => validate_create_user_args(args),
        CommandNames::DELETE_USER => validate_delete_user_args(args),
        CommandNames::GRANT => validate_grant_args(args),
        CommandNames::REVOKE => validate_revoke_args(args),
        CommandNames::CREATE_STORE => validate_create_store_args(args),
        CommandNames::LIST_KEYS => validate_list_keys_args(args),
    }
}

fn validate_set_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    if args.len() >= 3 {
        match DataTypes::from_str(&args[2]) {
            Ok(data_type) => {
                if data_type == DataTypes::STORE {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid data type. To create STORE use CREATE_STORE command",
                    ));
                }
                match data_type.validate_data(&args[1]) {
                    Ok(_) => {}
                    Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
                }
            }
            Err(e) => return Err(Error::new(ErrorKind::InvalidInput, e)),
        };
    }
    return Ok(());
}

fn validate_get_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}

fn validate_del_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}

fn validate_auth_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}

fn validate_get_user_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}

fn validate_create_user_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}

fn validate_delete_user_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}

fn validate_grant_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}

fn validate_revoke_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}

fn validate_create_store_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    if args[0] == "." {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Forbidden store name! {}", args[0]),
        ));
    }
    Ok(())
}

fn validate_list_keys_args(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 1 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Invalid number of arguments",
        ));
    }
    Ok(())
}
