use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UserCommand<'a> {
    user: &'a str,
    mode: u16,
    realname: &'a str,
}

impl<'a> UserCommand<'a> {
    pub fn new(user: &'a str, mode: u16, realname: &'a str) -> UserCommand<'a> {
        debug_assert!((mode & !0x6) == 0, "Only bit 2 and 3 having any signification");

        UserCommand {
            user: user,
            mode: mode,
            realname: realname,
        }
    }

    pub fn user(&'a self) -> &'a str {
        &self.user
    }

    pub fn mode(&self) -> u16 {
        self.mode
    }

    pub fn realname(&'a self) -> &'a str {
        &self.realname
    }
}

impl<'a> fmt::Display for UserCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} * :{}", CMD_USER, self.user, self.mode, self.realname)
    }
}

impl<'a> IrcMessage<'a> for UserCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<UserCommand<'a>, ParseMessageError> {
        debug_assert!(raw.command() == CMD_USER);

        let mut params = raw.parameters();
        let (user, mode, realname) = match (params.next(), params.next(), params.next(), params.next()) {
            (Some(user), Some(mode), _, Some(realname)) => {
                let mode = match mode.parse::<u16>() {
                    Ok(m) => m,
                    Err(..) => {
                        return Err(ParseMessageError::new(ParseMessageErrorKind::InvalidParam,
                                                          "Invalid mode"));
                    }
                };

                (user, mode, realname)
            },
            _ => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "USER requires 3 parameters"));
            }
        };

        Ok(UserCommand::new(user, mode, realname))
    }
}
