use std::fmt;

use protocol::command::CMD_PONG;
use protocol::message::{IrcMessage, MessageParamIter, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PongCommand<'a> {
    servers: &'a str,
}

impl<'a> PongCommand<'a> {
    pub fn new(servers: &'a str) -> PongCommand<'a> {
        PongCommand {
            servers: servers,
        }
    }

    pub fn servers(&self) -> MessageParamIter<'a> {
        MessageParamIter::wrap(self.servers)
    }
}

impl<'a> fmt::Display for PongCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", CMD_PONG, self.servers)
    }
}

impl<'a> IrcMessage<'a> for PongCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<PongCommand<'a>, ParseMessageError> {
        Ok(PongCommand::new(raw.parameters().get()))
    }
}
