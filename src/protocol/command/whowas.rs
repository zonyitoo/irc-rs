use std::fmt;

use protocol::command::{CMD_WHOWAS, MultipleFieldIter};
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WhowasCommand<'a> {
    nicknames: &'a str,
    count: Option<u32>,
    target: Option<&'a str>,
}

impl<'a> WhowasCommand<'a> {
    fn new(nicknames: &'a str, count_with_target: Option<(u32, Option<&'a str>)>) -> WhowasCommand<'a> {
        let (cnt, t) =
            match count_with_target {
                None => (None, None),
                Some((c, t)) => (Some(c), t)
            };

        WhowasCommand {
            nicknames: nicknames,
            count: cnt,
            target: t,
        }
    }

    pub fn nicknames(&self) -> MultipleFieldIter<'a> {
        MultipleFieldIter::wrap(self.nicknames)
    }

    pub fn count(&self) -> Option<u32> {
        self.count
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for WhowasCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_WHOWAS, self.nicknames));

        match self.count {
            None => Ok(()),
            Some(c) => {
                match self.target {
                    None => write!(f, " {}", c),
                    Some(t) => write!(f, " {} {}", c, t),
                }
            }
        }
    }
}

impl<'a> IrcMessage<'a> for WhowasCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<WhowasCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        let (nm, c, t) =
            match (params.next(), params.next(), params.next()) {
                (Some(nm), Some(c), t) => {
                    let c = match c.parse::<u32>() {
                        Err(..) => {
                            return Err(ParseMessageError::new(ParseMessageErrorKind::InvalidParam,
                                                              "Count should be a valid number"));
                        },
                        Ok(n) => n,
                    };

                    (nm, Some(c), t)
                },
                (Some(nm), None, _) => (nm, None, None),
                _ => {
                    return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                      "WHOWAS needs at least 1 parameter"));
                }
            };

        let cnt = match c {
            None => None,
            Some(c) => Some((c, t)),
        };

        Ok(WhowasCommand::new(nm, cnt))
    }
}
