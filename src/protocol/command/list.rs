use std::fmt;

use protocol::command::CMD_LIST;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListCommand<'a> {
    channels: Option<&'a str>,
    target: Option<&'a str>,
}

impl<'a> ListCommand<'a> {
    pub fn new(channels_with_target: Option<(&'a str, Option<&'a str>)>) -> ListCommand<'a> {
        let (chan, tar) =
            match channels_with_target {
                None => (None, None),
                Some((chan, tar)) => (Some(chan), tar)
            };

        ListCommand {
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

impl<'a> fmt::Display for ListCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_LIST));
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

impl<'a> IrcMessage<'a> for ListCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<ListCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let chan = match params.next() {
            None => None,
            Some(chan) => Some((chan, params.next())),
        };

        Ok(ListCommand::new(chan))
    }
}
