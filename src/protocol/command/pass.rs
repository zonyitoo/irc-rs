use std::fmt;

use protocol::command::CMD_PASS;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

/// The `PASS` command is used to set a 'connection password'.  The
/// optional password can and MUST be set before any attempt to register
/// the connection is made.  Currently this requires that user send a
/// `PASS` command before sending the `NICK`/`USER` combination.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PassCommand<'a> {
    password: &'a str,
}

impl<'a> PassCommand<'a> {
    pub fn new(pass: &'a str) -> PassCommand<'a> {
        PassCommand {
            password: pass,
        }
    }

    pub fn password(&'a self) -> &'a str {
        &self.password
    }
}

impl<'a> fmt::Display for PassCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", CMD_PASS, self.password)
    }
}

impl<'a> IrcMessage<'a> for PassCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<PassCommand<'a>, ParseMessageError> {
        debug_assert!(raw.command() == CMD_PASS);

        let mut params = raw.parameters();
        match params.next() {
            None => Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                               "PASS command needs one parameter")),
            Some(pwd) => Ok(PassCommand::new(pwd)),
        }
    }
}
