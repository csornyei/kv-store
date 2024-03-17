use std::io::Error;
use std::str::FromStr;

use super::parser::{parse_line, parse_permissions};
use super::validator::validate_args;
use super::CommandNames;

use crate::data::DataTypes;

pub struct Command {
    pub name: CommandNames,
    pub args: Vec<String>,
}

impl Command {
    fn new(name: CommandNames, args: Vec<String>) -> Command {
        match name {
            CommandNames::CREATE_USER => {
                return Command::new_create_user_command(name, args);
            }
            CommandNames::GRANT | CommandNames::REVOKE => {
                return Command::new_auth_command(name, args);
            }
            CommandNames::SET => {
                return Command::new_set_command(name, args);
            }
            _ => Command { name, args },
        }
    }

    fn new_create_user_command(name: CommandNames, args: Vec<String>) -> Command {
        let password = args[1].clone();
        let other_args = args[2..].join(" ");

        let permissions = parse_permissions(&other_args);

        let args = vec![args[0].clone(), password, permissions.to_string()];

        Command { name, args }
    }

    fn new_auth_command(name: CommandNames, args: Vec<String>) -> Command {
        let other_args = args[1..].join(" ");

        let permissions = parse_permissions(&other_args);

        let args = vec![args[0].clone(), permissions.to_string()];

        Command { name, args }
    }

    fn new_set_command(name: CommandNames, args: Vec<String>) -> Command {
        let key = args[0].clone();

        let value = args[1].clone();

        let data_type = if args.len() == 2 {
            DataTypes::STRING
        } else {
            DataTypes::from_str(&args[2]).unwrap()
        };

        Command {
            name,
            args: vec![key, value, data_type.to_string()],
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, args) = parse_line(s.to_string())?;
        validate_args(&name, args.clone())?;
        Ok(Command::new(name, args))
    }
}

#[cfg(test)]
mod command_tests {
    use super::*;

    #[test]
    fn test_command_from_str() {
        assert!(Command::from_str("GET key").is_ok());
        assert!(Command::from_str("GET key value").is_err());
    }
}
