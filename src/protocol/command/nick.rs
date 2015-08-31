use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

/// `NICK` command is used to give user a nickname or change the existing
/// one.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NickCommand<'a> {
    nickname: &'a str,
}

impl<'a> NickCommand<'a> {
    pub fn new(nick: &'a str) -> NickCommand<'a> {
        NickCommand {
            nickname: nick,
        }
    }

    pub fn nickname(&'a self) -> &'a str {
        &self.nickname
    }
}

impl<'a> fmt::Display for NickCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", CMD_NICK, self.nickname)
    }
}

impl<'a> IrcMessage<'a> for NickCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<NickCommand<'a>, ParseMessageError> {
        debug_assert!(raw.command() == CMD_NICK);

        let mut params = raw.parameters();
        match params.next() {
            None => Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                               "NICK command needs one parameter")),
            Some(pwd) => Ok(NickCommand::new(pwd)),
        }
    }
}
