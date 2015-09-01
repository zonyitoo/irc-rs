use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KickCommand<'a> {
    channels: &'a str,
    users: &'a str,
    comment: Option<&'a str>,
}

impl<'a> KickCommand<'a> {
    pub fn new(channels: &'a str, users: &'a str, comment: Option<&'a str>) -> KickCommand<'a> {
        KickCommand {
            channels: channels,
            users: users,
            comment: comment,
        }
    }

    pub fn channels(&self) -> MultipleFieldIter<'a> {
        MultipleFieldIter::wrap(self.channels)
    }

    pub fn users(&self) -> MultipleFieldIter<'a> {
        MultipleFieldIter::wrap(self.users)
    }

    pub fn comment(&self) -> Option<&'a str> {
        self.comment
    }
}

impl<'a> fmt::Display for KickCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {} {}", CMD_KICK, self.channels, self.users));
        match self.comment {
            None => Ok(()),
            Some(t) => write!(f, " :{}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for KickCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<KickCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        let (chans, users) = match (params.next(), params.next()) {
            (Some(c), Some(u)) => (c, u),
            _ => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "KICK requires at least 2 parameters"));
            },
        };

        Ok(KickCommand::new(chans, users, params.next()))
    }
}
