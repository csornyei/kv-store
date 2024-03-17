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
