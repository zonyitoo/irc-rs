use std::fmt;

use protocol::command::CMD_WHO;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WhoCommand<'a> {
    mask: Option<&'a str>,
    operator_only: bool,
}

impl<'a> WhoCommand<'a> {
    pub fn new(mask: Option<(&'a str, bool)>) -> WhoCommand<'a> {
        let (m, o) = match mask {
            None => (None, false),
            Some((m, o)) => (Some(m), o),
        };

        WhoCommand {
            mask: m,
            operator_only: o,
        }
    }

    pub fn mask(&self) -> Option<&'a str> {
        self.mask
    }

    pub fn is_operator_only(&self) -> bool {
        self.operator_only
    }
}

impl<'a> fmt::Display for WhoCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_WHO));
        match self.mask {
            None => Ok(()),
            Some(m) => {
                if self.operator_only {
                    write!(f, " {} o", m)
                } else {
                    write!(f, " {}", m)
                }
            }
        }
    }
}

impl<'a> IrcMessage<'a> for WhoCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<WhoCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let mask = match (params.next(), params.next()) {
            (Some(m), Some(op)) => {
                if op != "o" {
                    return Err(ParseMessageError::new(ParseMessageErrorKind::InvalidParam,
                                                      "WHO only accepts \"o\" as the last parameter"));
                }

                Some((m, true))
            },
            (Some(m), None) => Some((m, false)),
            _ => None,
        };

        Ok(WhoCommand::new(mask))
    }
}
