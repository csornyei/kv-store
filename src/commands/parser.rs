use std::fmt::{self, Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum CommandNames {
    SET,
    GET,
    DEL,

    // Authentication commands
    AUTH,
    GET_USER,
    CREATE_USER,
    DELETE_USER,

    // Authorization commands
    GRANT,
    REVOKE,
}

impl Display for CommandNames {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CommandNames::SET => write!(f, "SET"),
            CommandNames::GET => write!(f, "GET"),
            CommandNames::DEL => write!(f, "DEL"),
            CommandNames::AUTH => write!(f, "AUTH"),
            CommandNames::GET_USER => write!(f, "GET_USER"),
            CommandNames::CREATE_USER => write!(f, "CREATE_USER"),
            CommandNames::DELETE_USER => write!(f, "DELETE_USER"),
            CommandNames::GRANT => write!(f, "GRANT"),
            CommandNames::REVOKE => write!(f, "REVOKE"),
        }
    }
}

impl FromStr for CommandNames {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SET" => Ok(CommandNames::SET),
            "GET" => Ok(CommandNames::GET),
            "DEL" => Ok(CommandNames::DEL),
            "AUTH" => Ok(CommandNames::AUTH),
            "GET_USER" => Ok(CommandNames::GET_USER),
            "CREATE_USER" => Ok(CommandNames::CREATE_USER),
            "DELETE_USER" => Ok(CommandNames::DELETE_USER),
            "GRANT" => Ok(CommandNames::GRANT),
            "REVOKE" => Ok(CommandNames::REVOKE),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid command;")),
        }
    }
}

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
            _ => Command { name, args },
        }
    }

    fn new_create_user_command(name: CommandNames, args: Vec<String>) -> Command {
        let password = args[1].clone();
        let other_args = args[2..].join(" ");

        let permissions = Command::parse_permissions(&other_args);

        let args = vec![args[0].clone(), password, permissions.to_string()];

        Command { name, args }
    }

    fn new_auth_command(name: CommandNames, args: Vec<String>) -> Command {
        let other_args = args[1..].join(" ");

        let permissions = Command::parse_permissions(&other_args);

        let args = vec![args[0].clone(), permissions.to_string()];

        Command { name, args }
    }

    fn validate_args(command: &CommandNames, args: Vec<String>) -> Result<(), Error> {
        match command {
            CommandNames::SET => {
                if args.len() != 2 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::GET => {
                if args.len() != 1 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::DEL => {
                if args.len() != 1 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::AUTH => {
                if args.len() != 2 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::GET_USER => {
                if args.len() != 1 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::CREATE_USER => {
                if args.len() < 2 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::DELETE_USER => {
                if args.len() != 1 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::GRANT => {
                if args.len() < 2 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::REVOKE => {
                if args.len() < 2 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
        }
        Ok(())
    }

    fn parse_permissions(args: &str) -> u8 {
        let mut permissions = 0;
        for permission in args.split(' ') {
            match Command::parse_permission_num(permission) {
                Ok(perm) => return perm,
                Err(_) => {
                    let parsed_permissions =
                        Command::parse_permissions_str(permissions, permission);
                    match parsed_permissions {
                        Ok(perm) => {
                            permissions = perm;
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
            }
        }
        permissions
    }

    fn parse_permission_num(permissions: &str) -> Result<u8, Error> {
        match permissions.parse::<u8>() {
            Ok(perm) => Ok(perm),
            Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Invalid permission")),
        }
    }

    fn parse_permissions_str(current_permissions: u8, permission: &str) -> Result<u8, Error> {
        match permission {
            "SET" => Ok(current_permissions | 1 << 0),
            "GET" => Ok(current_permissions | 1 << 1),
            "DEL" => Ok(current_permissions | 1 << 2),
            "USER_ADMIN" => Ok(current_permissions | 1 << 3),

            _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid permission")),
        }
    }
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, args) = parse_line(s.to_string())?;
        Command::validate_args(&name, args.clone())?;
        Ok(Command::new(name, args))
    }
}

fn parse_line(line: String) -> Result<(CommandNames, Vec<String>), Error> {
    let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
    if parts.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "No command"));
    }
    if parts.len() < 2 {
        let cmd = CommandNames::from_str(parts[0])?;
        return Ok((cmd, Vec::new()));
    }

    let command = parts[0].to_string();
    let args: Vec<String> = parts[1].trim().split(' ').map(|s| s.to_string()).collect();

    let cmd = CommandNames::from_str(&command)?;
    Ok((cmd, args))
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_string_to_commands() {
        assert_eq!(CommandNames::from_str("SET").unwrap(), CommandNames::SET);
        assert_eq!(CommandNames::from_str("GET").unwrap(), CommandNames::GET);
        assert_eq!(CommandNames::from_str("DEL").unwrap(), CommandNames::DEL);
        assert!(CommandNames::from_str("INVALID").is_err());
    }

    #[test]
    fn test_parse_line_no_string() {
        assert!(parse_line("".to_string()).is_err());
    }

    #[test]
    fn test_parse_line_no_args() {
        assert_eq!(
            parse_line("GET".to_string()).unwrap(),
            (CommandNames::GET, Vec::new())
        );
    }

    #[test]
    fn test_parse_line_with_args() {
        assert_eq!(
            parse_line("SET key value".to_string()).unwrap(),
            (
                CommandNames::SET,
                vec!["key".to_string(), "value".to_string()]
            )
        );
    }

    #[test]
    fn test_command_validate_args() {
        assert!(Command::validate_args(&CommandNames::SET, vec!["key".to_string()]).is_err());
        assert!(Command::validate_args(
            &CommandNames::SET,
            vec!["key".to_string(), "value".to_string()]
        )
        .is_ok());
        assert!(Command::validate_args(
            &CommandNames::GET,
            vec!["key".to_string(), "value".to_string()]
        )
        .is_err());
        assert!(Command::validate_args(&CommandNames::GET, vec!["key".to_string()]).is_ok());
        assert!(Command::validate_args(
            &CommandNames::DEL,
            vec!["key".to_string(), "value".to_string()]
        )
        .is_err());
        assert!(Command::validate_args(&CommandNames::DEL, vec!["key".to_string()]).is_ok());
    }

    #[test]
    fn test_command_from_str() {
        assert!(Command::from_str("SET key value").is_ok());
        assert!(Command::from_str("SET key").is_err());
    }

    #[test]
    fn test_parse_permissions() {
        let permissions = Command::parse_permissions("");
        assert_eq!(permissions, 0);

        let permissions = Command::parse_permissions("SET");
        assert_eq!(permissions, 1);

        let permissions = Command::parse_permissions("SET GET");
        assert_eq!(permissions, 3);

        let permissions = Command::parse_permissions("SET GET DEL");

        assert_eq!(permissions, 7);

        let permissions = Command::parse_permissions("SET GET DEL USER_ADMIN");
        assert_eq!(permissions, 15);

        let permissions = Command::parse_permissions("SET GET DEL USER_ADMIN INVALID");
        assert_eq!(permissions, 15);

        let permissions = Command::parse_permissions("255");
        assert_eq!(permissions, 255);
    }
}
