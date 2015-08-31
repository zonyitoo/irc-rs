use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NoticeCommand<'a> {
    msgtarget: &'a str,
    msg: &'a str,
}

impl<'a> NoticeCommand<'a> {
    pub fn new(msgtarget: &'a str, msg: &'a str) -> NoticeCommand<'a> {
        NoticeCommand {
            msgtarget: msgtarget,
            msg: msg,
        }
    }

    pub fn target(&self) -> &'a str {
        self.msgtarget
    }

    pub fn message(&self) -> &'a str {
        self.msg
    }
}

impl<'a> fmt::Display for NoticeCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} :{}", CMD_NOTICE, self.msgtarget, self.msg)
    }
}

impl<'a> IrcMessage<'a> for NoticeCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<NoticeCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        let target = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "NOTICE requires a target"));
            },
            Some(t) => t,
        };

        let msg = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "NOTICE requires a message"));
            },
            Some(m) => m,
        };

        Ok(NoticeCommand::new(target, msg))
    }
}
