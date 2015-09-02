use std::fmt;

use protocol::command::CMD_PRIVMSG;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrivmsgCommand<'a> {
    msgtarget: &'a str,
    msg: &'a str,
}

impl<'a> PrivmsgCommand<'a> {
    pub fn new(msgtarget: &'a str, msg: &'a str) -> PrivmsgCommand<'a> {
        PrivmsgCommand {
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

impl<'a> fmt::Display for PrivmsgCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} :{}", CMD_PRIVMSG, self.msgtarget, self.msg)
    }
}

impl<'a> IrcMessage<'a> for PrivmsgCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<PrivmsgCommand<'a>, ParseMessageError> {
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

        Ok(PrivmsgCommand::new(target, msg))
    }
}
