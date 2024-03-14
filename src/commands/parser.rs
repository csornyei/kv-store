use std::fmt::{self, Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum CommandNames {
    SET,
    GET,
    DEL,
}

impl Display for CommandNames {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            CommandNames::SET => write!(f, "SET"),
            CommandNames::GET => write!(f, "GET"),
            CommandNames::DEL => write!(f, "DEL"),
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
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid command")),
        }
    }
}

pub struct Command {
    pub name: CommandNames,
    pub args: Vec<String>,
}

impl Command {
    fn new(name: CommandNames, args: Vec<String>) -> Command {
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
        }
        Ok(())
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
}
