use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InfoCommand<'a> {
    target: Option<&'a str>,
}

impl<'a> InfoCommand<'a> {
    pub fn new(target: Option<&'a str>) -> InfoCommand<'a> {
        InfoCommand {
            target: target,
        }
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for InfoCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_INFO));
        match self.target {
            None => Ok(()),
            Some(t) => write!(f, " {}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for InfoCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<InfoCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        Ok(InfoCommand::new(params.next()))
    }
}
