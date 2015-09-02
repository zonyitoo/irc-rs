use std::fmt;

use protocol::command::CMD_USERS;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UsersCommand<'a> {
    target: Option<&'a str>,
}

impl<'a> UsersCommand<'a> {
    pub fn new(target: Option<&'a str>) -> UsersCommand<'a> {
        UsersCommand {
            target: target,
        }
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for UsersCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_USERS));
        match self.target {
            None => Ok(()),
            Some(t) => write!(f, " {}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for UsersCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<UsersCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        Ok(UsersCommand::new(params.next()))
    }
}
