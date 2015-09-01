use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ServlistCommand<'a> {
    mask: Option<&'a str>,
    stype: Option<&'a str>,
}

impl<'a> ServlistCommand<'a> {
    pub fn new(mask_with_type: Option<(&'a str, Option<&'a str>)>) -> ServlistCommand<'a> {
        let (chan, tar) =
            match mask_with_type {
                None => (None, None),
                Some((chan, tar)) => (Some(chan), tar)
            };

        ServlistCommand {
            mask: chan,
            stype: tar,
        }
    }

    pub fn mask(&self) -> Option<&'a str> {
        self.mask
    }

    pub fn server_type(&self) -> Option<&'a str> {
        self.stype
    }
}

impl<'a> fmt::Display for ServlistCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_NAMES));
        match self.mask {
            None => Ok(()),
            Some(chan) => {
                match self.stype {
                    None => write!(f, " {}", chan),
                    Some(t) => write!(f, " {} {}", chan, t),
                }
            }
        }
    }
}

impl<'a> IrcMessage<'a> for ServlistCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<ServlistCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let chan = match params.next() {
            None => None,
            Some(chan) => Some((chan, params.next())),
        };

        Ok(ServlistCommand::new(chan))
    }
}
