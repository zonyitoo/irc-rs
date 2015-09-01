use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DieCommand;

impl fmt::Display for DieCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", CMD_DIE)
    }
}

impl<'a> IrcMessage<'a> for DieCommand {
    fn from_raw(_raw: &RawMessage<'a>) -> Result<DieCommand, ParseMessageError> {
        Ok(DieCommand)
    }
}
