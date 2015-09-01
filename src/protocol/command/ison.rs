use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, MessageParamIter, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IsonCommand<'a> {
    nicknames: &'a str,
}

impl<'a> IsonCommand<'a> {
    pub fn new(nicknames: &'a str) -> IsonCommand<'a> {
        IsonCommand {
            nicknames: nicknames,
        }
    }

    pub fn nicknames(&self) -> MessageParamIter<'a> {
        MessageParamIter::wrap(self.nicknames)
    }
}

impl<'a> fmt::Display for IsonCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", CMD_ISON, self.nicknames)
    }
}

impl<'a> IrcMessage<'a> for IsonCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<IsonCommand<'a>, ParseMessageError> {
        let nicknames = raw.parameters().get();

        if nicknames.is_empty() {
            return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                              "ISON requires at least one nickname"));
        }

        Ok(IsonCommand::new(nicknames))
    }
}
