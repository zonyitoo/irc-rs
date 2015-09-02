use std::fmt;

use protocol::command::CMD_STATS;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StatsCommand<'a> {
    query: Option<&'a str>,
    target: Option<&'a str>,
}

impl<'a> StatsCommand<'a> {
    pub fn new(query_with_target: Option<(&'a str, Option<&'a str>)>) -> StatsCommand<'a> {
        let (query, tar) =
            match query_with_target {
                None => (None, None),
                Some((query, tar)) => (Some(query), tar)
            };

        StatsCommand {
            query: query,
            target: tar,
        }
    }

    pub fn query(&self) -> Option<&'a str> {
        self.query
    }

    pub fn target(&self) -> Option<&'a str> {
        self.target
    }
}

impl<'a> fmt::Display for StatsCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", CMD_STATS));
        match self.query {
            None => Ok(()),
            Some(q) => {
                match self.target {
                    None => write!(f, " {}", q),
                    Some(t) => write!(f, " {} {}", q, t),
                }
            }
        }
    }
}

impl<'a> IrcMessage<'a> for StatsCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<StatsCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let q = match params.next() {
            None => None,
            Some(q) => Some((q, params.next())),
        };

        Ok(StatsCommand::new(q))
    }
}
