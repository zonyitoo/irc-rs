use std::fmt;

use protocol::command::CMD_ERROR;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ErrorCommand<'a> {
    msg: &'a str,
}

impl<'a> ErrorCommand<'a> {
    pub fn new(msg: &'a str) -> ErrorCommand<'a> {
        ErrorCommand {
            msg: msg,
        }
    }

    pub fn message(&self) -> &'a str {
        self.msg
    }
}

impl<'a> fmt::Display for ErrorCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} :{}", CMD_ERROR, self.msg)
    }
}

impl<'a> IrcMessage<'a> for ErrorCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<ErrorCommand<'a>, ParseMessageError> {
        let msg = match raw.parameters().next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "ERROR command needs a message"));
            },
            Some(m) => m,
        };

        Ok(ErrorCommand::new(msg))
    }
}
