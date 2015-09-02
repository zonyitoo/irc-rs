use std::fmt;

use protocol::command::CMD_LUSERS;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LusersCommand<'a> {
    mask: Option<&'a str>,
    target: Option<&'a str>,
}

impl<'a> LusersCommand<'a> {
    pub fn new(mask_with_target: Option<(&'a str, Option<&'a str>)>) -> LusersCommand<'a> {
        let (mask, tar) =
            match mask_with_target {
                None => (None, None),
                Some((mask, tar)) => (Some(mask), tar)
            };

        LusersCommand {
            mask: mask,
            target: tar,
        }
    }

    pub fn mask(&self) -> Option<&'a str> {
        self.mask
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for LusersCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_LUSERS));
        match self.mask {
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

impl<'a> IrcMessage<'a> for LusersCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<LusersCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let chan = match params.next() {
            None => None,
            Some(chan) => Some((chan, params.next())),
        };

        Ok(LusersCommand::new(chan))
    }
}
