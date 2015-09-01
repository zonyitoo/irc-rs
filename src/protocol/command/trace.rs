use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TraceCommand<'a> {
    target: Option<&'a str>,
}

impl<'a> TraceCommand<'a> {
    pub fn new(target: Option<&'a str>) -> TraceCommand<'a> {
        TraceCommand {
            target: target,
        }
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for TraceCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_TRACE));
        match self.target {
            None => Ok(()),
            Some(t) => write!(f, " {}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for TraceCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<TraceCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        Ok(TraceCommand::new(params.next()))
    }
}
