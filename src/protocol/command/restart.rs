use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RestartCommand;

impl fmt::Display for RestartCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", CMD_RESTART)
    }
}

impl<'a> IrcMessage<'a> for RestartCommand {
    fn from_raw(_raw: &RawMessage<'a>) -> Result<RestartCommand, ParseMessageError> {
        Ok(RestartCommand)
    }
}
