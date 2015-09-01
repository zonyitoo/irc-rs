use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TopicCommand<'a> {
    channel: &'a str,
    topic: Option<&'a str>,
}

impl<'a> TopicCommand<'a> {
    pub fn new(channel: &'a str, topic: Option<&'a str>) -> TopicCommand<'a> {
        TopicCommand {
            channel: channel,
            topic: topic,
        }
    }

    pub fn channel(&self) -> &'a str {
        self.channel
    }

    pub fn topic(&self) -> Option<&'a str> {
        self.topic
    }
}

impl<'a> fmt::Display for TopicCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_TOPIC, self.channel));
        match self.topic {
            None => Ok(()),
            Some(t) => write!(f, " :{}", t),
        }
    }
}

impl<'a> IrcMessage<'a> for TopicCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<TopicCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();
        let channel = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "TOPIC requires a channel"));
            },
            Some(t) => t,
        };

        Ok(TopicCommand::new(channel, params.next()))
    }
}
