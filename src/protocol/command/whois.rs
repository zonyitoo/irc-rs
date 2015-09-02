use std::fmt;

use protocol::command::{CMD_WHOIS, MultipleFieldIter};
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WhoisCommand<'a> {
    target: Option<&'a str>,
    masks: &'a str,
}

impl<'a> WhoisCommand<'a> {
    pub fn new(target: Option<&'a str>, masks: &'a str) -> WhoisCommand<'a> {
        WhoisCommand {
            target: target,
            masks: masks,
        }
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }

    pub fn masks(&self) -> MultipleFieldIter<'a> {
        MultipleFieldIter::wrap(self.masks)
    }
}

impl<'a> fmt::Display for WhoisCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_WHOIS));

        if let Some(t) = self.target {
            try!(write!(f, " {}", t));
        }

        write!(f, " {}", self.masks)
    }
}

impl<'a> IrcMessage<'a> for WhoisCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<WhoisCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let (target, masks) = match (params.next(), params.next()) {
            (Some(t), Some(m)) => (Some(t), m),
            (Some(m), None) => (None, m),
            _ => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "WHOIS needs at least 1 parameter"));
            }
        };

        Ok(WhoisCommand::new(target, masks))
    }
}
