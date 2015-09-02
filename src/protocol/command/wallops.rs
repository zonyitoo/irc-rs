use std::fmt;

use protocol::command::CMD_WALLOPS;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WallopsCommand<'a> {
    msg: &'a str,
}

impl<'a> WallopsCommand<'a> {
    pub fn new(msg: &'a str) -> WallopsCommand<'a> {
        WallopsCommand {
            msg: msg,
        }
    }

    pub fn message(&self) -> &'a str {
        self.msg
    }
}

impl<'a> fmt::Display for WallopsCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} :{}", CMD_WALLOPS, self.msg)
    }
}

impl<'a> IrcMessage<'a> for WallopsCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<WallopsCommand<'a>, ParseMessageError> {
        let msg = match raw.parameters().next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "WALLOPS command needs a message"));
            },
            Some(m) => m,
        };

        Ok(WallopsCommand::new(msg))
    }
}
