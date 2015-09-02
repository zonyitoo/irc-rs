use std::fmt;

use protocol::command::CMD_NAMES;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NamesCommand<'a> {
    channels: Option<&'a str>,
    target: Option<&'a str>,
}

impl<'a> NamesCommand<'a> {
    pub fn new(channels_with_target: Option<(&'a str, Option<&'a str>)>) -> NamesCommand<'a> {
        let (chan, tar) =
            match channels_with_target {
                None => (None, None),
                Some((chan, tar)) => (Some(chan), tar)
            };

        NamesCommand {
            channels: chan,
            target: tar,
        }
    }

    pub fn channels(&self) -> Option<&'a str> {
        self.channels
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for NamesCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_NAMES));
        match self.channels {
            None => Ok(()),
            Some(chan) => {
                match self.target {
                    None => write!(f, " {}", chan),
                    Some(t) => write!(f, " {} {}", chan, t),
                }
            }
        }
    }
}

impl<'a> IrcMessage<'a> for NamesCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<NamesCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let chan = match params.next() {
            None => None,
            Some(chan) => Some((chan, params.next())),
        };

        Ok(NamesCommand::new(chan))
    }
}
