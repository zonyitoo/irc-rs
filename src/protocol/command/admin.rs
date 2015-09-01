use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AdminCommand<'a> {
    target: Option<&'a str>,
}

impl<'a> AdminCommand<'a> {
    pub fn new(target: Option<&'a str>) -> AdminCommand<'a> {
        AdminCommand {
            target: target,
        }
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for AdminCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_ADMIN));
        match self.target {
            None => Ok(()),
            Some(t) => write!(f, " {}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for AdminCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<AdminCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        Ok(AdminCommand::new(params.next()))
    }
}
