use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PartCommand<'a> {
    channels: &'a str,
    message: Option<&'a str>,
}

impl<'a> PartCommand<'a> {
    pub fn new(channels: &'a str, message: Option<&'a str>) -> PartCommand<'a> {
        PartCommand {
            channels: channels,
            message: message,
        }
    }

    pub fn channels(&self) -> MultipleFieldIter<'a> {
        MultipleFieldIter::wrap(self.channels)
    }

    pub fn message(&self) -> Option<&'a str> {
        self.message
    }
}

impl<'a> fmt::Display for PartCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_PART, self.channels));

        if let Some(msg) = self.message {
            write!(f, " {}", msg)
        } else {
            Ok(())
        }
    }
}

impl<'a> IrcMessage<'a> for PartCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<PartCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let chan = match params.next() {
            Some(c) => c,
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "PART command requires at least 1 command"));
            }
        };

        Ok(PartCommand::new(chan, params.next()))
    }
}
