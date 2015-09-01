use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ConnectCommand<'a> {
    target_server: &'a str,
    port: u16,
    remote_server: Option<&'a str>,
}

impl<'a> ConnectCommand<'a> {
    pub fn new(target: &'a str, port: u16, remote: Option<&'a str>) -> ConnectCommand<'a> {
        ConnectCommand {
            target_server: target,
            port: port,
            remote_server: remote,
        }
    }

    pub fn target_server(&self) -> &'a str {
        self.target_server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn remote_server(&self) -> Option<&'a str> {
        self.remote_server
    }
}

impl<'a> fmt::Display for ConnectCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {} {}", CMD_CONNECT, self.target_server, self.port));
        match self.remote_server {
            None => Ok(()),
            Some(t) => write!(f, " {}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for ConnectCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<ConnectCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        let (t, p) = match (params.next(), params.next()) {
            (Some(t), Some(p)) => {
                let p = match p.parse::<u16>() {
                    Err(..) => {
                        return Err(ParseMessageError::new(ParseMessageErrorKind::InvalidParam,
                                                          "Port should be a valid u16 number"));
                    },
                    Ok(p) => p,
                };

                (t, p)
            },
            _ => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "CONNECT requires at least 2 parameters"));
            },
        };

        Ok(ConnectCommand::new(t, p, params.next()))
    }
}
