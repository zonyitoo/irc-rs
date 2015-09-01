use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct VersionCommand<'a> {
    target: Option<&'a str>,
}

impl<'a> VersionCommand<'a> {
    pub fn new(target: Option<&'a str>) -> VersionCommand<'a> {
        VersionCommand {
            target: target,
        }
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for VersionCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_VERSION));
        match self.target {
            None => Ok(()),
            Some(t) => write!(f, " {}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for VersionCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<VersionCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        Ok(VersionCommand::new(params.next()))
    }
}
