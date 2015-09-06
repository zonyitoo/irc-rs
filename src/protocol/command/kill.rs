use std::fmt;

use protocol::command::CMD_KILL;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct KillCommand<'a> {
    nickname: &'a str,
    msg: Option<&'a str>,
}

impl<'a> KillCommand<'a> {
    fn new(nickname: &'a str, msg: Option<&'a str>) -> KillCommand<'a> {
        KillCommand {
            nickname: nickname,
            msg: msg,
        }
    }

    pub fn nickname(&self) -> &'a str {
        self.nickname
    }

    pub fn message(&self) -> Option<&'a str> {
        self.msg
    }
}

impl<'a> fmt::Display for KillCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_KILL, self.nickname));

        match self.msg {
            Some(m) => write!(f, " {}", m),
            None => Ok(()),
        }
    }
}

impl<'a> IrcMessage<'a> for KillCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<KillCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let (nick, msg) = match (params.next(), params.next()) {
            (Some(nick), Some(msg)) => (nick, Some(msg)),
            (Some(nick), None) => (nick, None),
            _ => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "KILL command needs at least 1 parameters"));
            }
        };

        Ok(KillCommand::new(nick, msg))
    }
}
