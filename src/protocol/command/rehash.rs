use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RehashCommand;

impl fmt::Display for RehashCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", CMD_REHASH)
    }
}

impl<'a> IrcMessage<'a> for RehashCommand {
    fn from_raw(_raw: &RawMessage<'a>) -> Result<RehashCommand, ParseMessageError> {
        Ok(RehashCommand)
    }
}
