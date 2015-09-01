use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MotdCommand<'a> {
    target: Option<&'a str>,
}

impl<'a> MotdCommand<'a> {
    pub fn new(target: Option<&'a str>) -> MotdCommand<'a> {
        MotdCommand {
            target: target,
        }
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for MotdCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_MOTD));
        match self.target {
            None => Ok(()),
            Some(t) => write!(f, " {}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for MotdCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<MotdCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        Ok(MotdCommand::new(params.next()))
    }
}
