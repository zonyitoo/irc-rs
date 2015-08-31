use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, MessageParamIter, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PingCommand<'a> {
    servers: &'a str,
}

impl<'a> PingCommand<'a> {
    pub fn new(servers: &'a str) -> PingCommand<'a> {
        PingCommand {
            servers: servers,
        }
    }

    pub fn servers(&self) -> MessageParamIter<'a> {
        MessageParamIter::wrap(self.servers)
    }
}

impl<'a> fmt::Display for PingCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", CMD_PING, self.servers)
    }
}

impl<'a> IrcMessage<'a> for PingCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<PingCommand<'a>, ParseMessageError> {
        Ok(PingCommand::new(raw.parameters().get()))
    }
}
