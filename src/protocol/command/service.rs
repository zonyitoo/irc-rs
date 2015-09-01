use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ServiceCommand<'a> {
    nickname: &'a str,
    distribution: &'a str,
    service_type: &'a str,
    info: &'a str,
}

impl<'a> ServiceCommand<'a> {
    pub fn new(nickname: &'a str, distribution: &'a str, service_type: &'a str, info: &'a str)
            -> ServiceCommand<'a> {
        ServiceCommand {
            nickname: nickname,
            distribution: distribution,
            service_type: service_type,
            info: info,
        }
    }

    pub fn nickname(&self) -> &'a str {
        self.nickname
    }

    pub fn distribution(&self) -> &'a str {
        self.distribution
    }

    pub fn service_type(&self) -> &'a str {
        self.service_type
    }

    pub fn info(&self) -> &'a str {
        self.info
    }
}

impl<'a> fmt::Display for ServiceCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} * {} {} * {}", CMD_SERVICE, self.nickname, self.distribution, self.service_type, self.info)
    }
}

impl<'a> IrcMessage<'a> for ServiceCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<ServiceCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let (nick, dist, service_type, info) =
            match (params.next(), params.next(), params.next(), params.next(), params.next(), params.next()) {
                (Some(nickname), _, Some(dist), Some(service_type), _, Some(info)) => (nickname, dist, service_type, info),
                _ => {
                    return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                      "SERVICE command needs 6 parameters"));
                }
            };

        Ok(ServiceCommand::new(nick, dist, service_type, info))
    }
}
