use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SummonCommand<'a> {
    user: &'a str,
    target: Option<&'a str>,
    channel: Option<&'a str>,
}

impl<'a> SummonCommand<'a> {
    fn new(user: &'a str, target_with_channel: Option<(&'a str, Option<&'a str>)>) -> SummonCommand<'a> {
        let (cnt, t) =
            match target_with_channel {
                None => (None, None),
                Some((c, t)) => (Some(c), t)
            };

        SummonCommand {
            user: user,
            target: cnt,
            channel: t,
        }
    }

    pub fn user(&self) -> MultipleFieldIter<'a> {
        MultipleFieldIter::wrap(self.user)
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }

    pub fn channel(&self) -> Option<&'a str> {
        self.channel
    }
}

impl<'a> fmt::Display for SummonCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_SUMMON, self.user));

        match self.target {
            None => Ok(()),
            Some(c) => {
                match self.channel {
                    None => write!(f, " {}", c),
                    Some(t) => write!(f, " {} {}", c, t),
                }
            }
        }
    }
}

impl<'a> IrcMessage<'a> for SummonCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<SummonCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        let (nm, c, t) =
            match (params.next(), params.next(), params.next()) {
                (Some(nm), Some(c), t) => (nm, Some(c), t),
                (Some(nm), None, _) => (nm, None, None),
                _ => {
                    return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                      "SUMMON needs at least 1 parameter"));
                }
            };

        let cnt = match c {
            None => None,
            Some(c) => Some((c, t)),
        };

        Ok(SummonCommand::new(nm, cnt))
    }
}
