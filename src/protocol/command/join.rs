use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JoinCommand<'a> {
    channels: &'a str,
    keys: Option<&'a str>,
}

impl<'a> JoinCommand<'a> {
    pub fn new(c: &'a str, keys: Option<&'a str>) -> JoinCommand<'a> {
        JoinCommand {
            channels: c,
            keys: keys,
        }
    }

    pub fn channels(&self) -> ChannelIter<'a> {
        ChannelIter::wrap(self.channels)
    }

    pub fn keys(&self) -> Option<&'a str> {
        self.keys
    }
}

impl<'a> fmt::Display for JoinCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_JOIN, self.channels));

        match self.keys {
            Some(k) => write!(f, "{}", k),
            None => Ok(()),
        }
    }
}

impl<'a> IrcMessage<'a> for JoinCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<JoinCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let channels = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "JOIN requires channels"));
            },
            Some(n) => n
        };

        let keys = params.next();

        Ok(JoinCommand::new(channels, keys))
    }
}
