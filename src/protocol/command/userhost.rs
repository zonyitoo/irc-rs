use std::fmt;

use protocol::command::CMD_USERHOST;
use protocol::message::{IrcMessage, MessageParamIter, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserhostCommand<'a> {
    nicknames: &'a str,
}

impl<'a> UserhostCommand<'a> {
    pub fn new(nicknames: &'a str) -> UserhostCommand<'a> {
        UserhostCommand {
            nicknames: nicknames,
        }
    }

    pub fn nicknames(&self) -> MessageParamIter<'a> {
        MessageParamIter::wrap(self.nicknames)
    }
}

impl<'a> fmt::Display for UserhostCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", CMD_USERHOST, self.nicknames)
    }
}

impl<'a> IrcMessage<'a> for UserhostCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<UserhostCommand<'a>, ParseMessageError> {
        let nicknames = raw.parameters().get();

        if nicknames.is_empty() {
            return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                              "USERHOST requires at least one nickname"));
        }

        Ok(UserhostCommand::new(nicknames))
    }
}
