use std::fmt;

use protocol::command::CMD_MODE;
use protocol::message::{IrcMessage, MessageParamIter, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ModeCommand<'a> {
    target: &'a str,
    modes: &'a str,
}

impl<'a> ModeCommand<'a> {
    pub fn new(target: &'a str, modes: &'a str) -> ModeCommand<'a> {
        ModeCommand {
            target: target,
            modes: modes,
        }
    }

    pub fn target(&self) -> &'a str {
        self.target
    }

    pub fn modes(&self) -> MessageParamIter<'a> {
        MessageParamIter::wrap(self.modes)
    }
}

impl<'a> fmt::Display for ModeCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_MODE, self.target));

        if self.modes.is_empty() {
            Ok(())
        } else {
            write!(f, " {}", self.modes)
        }
    }
}

impl<'a> IrcMessage<'a> for ModeCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<ModeCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let target = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "MODE command needs a target"));
            },
            Some(t) => t,
        };

        Ok(ModeCommand::new(target, params.get()))
    }
}
