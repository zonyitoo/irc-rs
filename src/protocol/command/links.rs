use std::fmt;

use protocol::command::CMD_LINKS;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LinksCommand<'a> {
    remote: Option<&'a str>,
    mask: Option<&'a str>,
}

impl<'a> LinksCommand<'a> {
    pub fn new(remote_with_mask: Option<(Option<&'a str>, &'a str)>) -> LinksCommand<'a> {
        let (remote, tar) =
            match remote_with_mask {
                None => (None, None),
                Some((remote, tar)) => (remote, Some(tar))
            };

        LinksCommand {
            remote: remote,
            mask: tar,
        }
    }

    pub fn remote(&self) -> Option<&'a str> {
        self.remote
    }

    pub fn mask(&self) -> Option<&'a str> {
        self.mask
    }
}

impl<'a> fmt::Display for LinksCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_LINKS));
        match self.mask {
            None => Ok(()),
            Some(q) => {
                match self.remote {
                    None => write!(f, " {}", q),
                    Some(t) => write!(f, " {} {}", t, q),
                }
            }
        }
    }
}

impl<'a> IrcMessage<'a> for LinksCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<LinksCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let q = match (params.next(), params.next()) {
            (None, _) => None,
            (Some(r), Some(m)) => Some((Some(r), m)),
            (Some(m), None) => Some((None, m)),
        };

        Ok(LinksCommand::new(q))
    }
}
