use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct QuitCommand<'a> {
    msg: Option<&'a str>,
}

impl<'a> QuitCommand<'a> {
    pub fn new(msg: Option<&'a str>) -> QuitCommand<'a> {
        QuitCommand {
            msg: msg,
        }
    }

    pub fn message(&self) -> Option<&'a str> {
        self.msg
    }
}

impl<'a> fmt::Display for QuitCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_QUIT));

        match self.msg {
            Some(m) => write!(f, " :{}", m),
            None => Ok(()),
        }
    }
}

impl<'a> IrcMessage<'a> for QuitCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<QuitCommand<'a>, ParseMessageError> {
        Ok(QuitCommand::new(raw.parameters().next()))
    }
}
