use std::fmt;

use protocol::command::CMD_SQUERY;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SqueryCommand<'a> {
    servicename: &'a str,
    text: &'a str,
}

impl<'a> SqueryCommand<'a> {
    pub fn new(servicename: &'a str, text: &'a str) -> SqueryCommand<'a> {
        SqueryCommand {
            servicename: servicename,
            text: text,
        }
    }

    pub fn service_name(&self) -> &'a str {
        self.servicename
    }

    pub fn text(&self) -> &'a str {
        self.text
    }
}

impl<'a> fmt::Display for SqueryCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} :{}", CMD_SQUERY, self.servicename, self.text)
    }
}

impl<'a> IrcMessage<'a> for SqueryCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<SqueryCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        let (sn, t) =
            match (params.next(), params.next()) {
                (Some(sn), Some(t)) => (sn, t),
                _ => {
                    return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                      "SQUERY needs 2 parameters"));
                }
            };

        Ok(SqueryCommand::new(sn, t))
    }
}
