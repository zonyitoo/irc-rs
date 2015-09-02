use std::fmt;

use protocol::command::CMD_TIME;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TimeCommand<'a> {
    target: Option<&'a str>,
}

impl<'a> TimeCommand<'a> {
    pub fn new(target: Option<&'a str>) -> TimeCommand<'a> {
        TimeCommand {
            target: target,
        }
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for TimeCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_TIME));
        match self.target {
            None => Ok(()),
            Some(t) => write!(f, " {}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for TimeCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<TimeCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        Ok(TimeCommand::new(params.next()))
    }
}
