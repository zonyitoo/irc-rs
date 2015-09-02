use std::fmt;

use protocol::command::CMD_SQUIT;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SQuitCommand<'a> {
    server: &'a str,
    msg: Option<&'a str>,
}

impl<'a> SQuitCommand<'a> {
    pub fn new(server: &'a str, msg: Option<&'a str>) -> SQuitCommand<'a> {
        SQuitCommand {
            server: server,
            msg: msg,
        }
    }

    pub fn server(&self) -> &'a str {
        self.server
    }

    pub fn message(&self) -> Option<&'a str> {
        self.msg
    }
}

impl<'a> fmt::Display for SQuitCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_SQUIT, self.server));

        match self.msg {
            Some(m) => write!(f, " :{}", m),
            None => Ok(()),
        }
    }
}

impl<'a> IrcMessage<'a> for SQuitCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<SQuitCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let (svr, msg) = match (params.next(), params.next()) {
            (Some(svr), Some(msg)) => (svr, Some(msg)),
            (Some(svr), None) => (svr, None),
            _ => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "SQUIT command needs at least 1 parameters"));
            }
        };

        Ok(SQuitCommand::new(svr, msg))
    }
}
