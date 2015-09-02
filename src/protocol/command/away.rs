use std::fmt;

use protocol::command::CMD_AWAY;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AwayCommand<'a> {
    msg: Option<&'a str>,
}

impl<'a> AwayCommand<'a> {
    pub fn new(msg: Option<&'a str>) -> AwayCommand<'a> {
        AwayCommand {
            msg: msg,
        }
    }

    pub fn message(&self) -> Option<&'a str> {
        self.msg
    }
}

impl<'a> fmt::Display for AwayCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_AWAY));

        match self.msg {
            Some(m) => write!(f, " :{}", m),
            None => Ok(()),
        }
    }
}

impl<'a> IrcMessage<'a> for AwayCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<AwayCommand<'a>, ParseMessageError> {
        Ok(AwayCommand::new(raw.parameters().next()))
    }
}
