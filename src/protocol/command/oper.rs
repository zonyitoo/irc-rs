use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OperCommand<'a> {
    name: &'a str,
    password: &'a str,
}

impl<'a> OperCommand<'a> {
    pub fn new(name: &'a str, password: &'a str) -> OperCommand<'a> {
        OperCommand {
            name: name,
            password: password,
        }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn password(&self) -> &'a str {
        self.password
    }
}

impl<'a> fmt::Display for OperCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", CMD_OPER, self.name, self.password)
    }
}

impl<'a> IrcMessage<'a> for OperCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<OperCommand<'a>, ParseMessageError> {
        let mut param = raw.parameters();

        let (name, pwd) = match (param.next(), param.next()) {
            (Some(name), Some(pwd)) => (name, pwd),
            _ => return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                   "OPER command requires 2 parameters")),
        };

        Ok(OperCommand::new(name, pwd))
    }
}
