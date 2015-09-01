use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InviteCommand<'a> {
    nickname: &'a str,
    channel: &'a str,
}

impl<'a> InviteCommand<'a> {
    pub fn new(nickname: &'a str, channel: &'a str) -> InviteCommand<'a> {
        InviteCommand {
            nickname: nickname,
            channel: channel,
        }
    }

    pub fn channel(&self) -> &'a str {
        self.channel
    }

    pub fn nickname(&self) -> &'a str {
        self.nickname
    }
}

impl<'a> fmt::Display for InviteCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", CMD_INVITE, self.nickname, self.channel)
    }
}

impl<'a> IrcMessage<'a> for InviteCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<InviteCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        let nickname = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "INVITE requires a nickname"));
            },
            Some(t) => t,
        };

        let channel = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "INVITE requires a channel"));
            },
            Some(t) => t,
        };

        Ok(InviteCommand::new(nickname, channel))
    }
}
