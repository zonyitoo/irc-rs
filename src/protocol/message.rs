
use std::fmt;
use std::iter::Iterator;
use std::str::FromStr;

use protocol::command::Command;
use protocol::reply::Reply;

pub trait IrcMessage<'a>: fmt::Display + Sized {
    fn from_raw(raw: &RawMessage<'a>) -> Result<Self, ParseMessageError>;

    fn from_str(s: &'a str) -> Result<Self, ParseMessageError> {
        let raw_msg = try!(RawMessage::from_str(s));
        Self::from_raw(&raw_msg)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseMessageErrorKind {
    NeedMoreParams,
    InvalidParam,
    InvalidReplyCode,
    UnrecognizedCommand,
    UnrecognizedReply,
    MissingPrefix,
}

pub struct ParseMessageError {
    kind: ParseMessageErrorKind,
    desc: &'static str,
    detail: Option<String>,
}

impl ParseMessageError {
    pub fn new_with_detail(kind: ParseMessageErrorKind, desc: &'static str, detail: String) -> ParseMessageError {
        ParseMessageError {
            kind: kind,
            desc: desc,
            detail: Some(detail),
        }
    }

    pub fn new(kind: ParseMessageErrorKind, desc: &'static str) -> ParseMessageError {
        ParseMessageError {
            kind: kind,
            desc: desc,
            detail: None,
        }
    }

    pub fn kind(&self) -> ParseMessageErrorKind {
        self.kind
    }

    pub fn desc(&self) -> &'static str {
        self.desc
    }

    pub fn detail<'a>(&'a self) -> Option<&'a str> {
        self.detail.as_ref().map(|x| &x[..])
    }
}

impl fmt::Debug for ParseMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{:?} {}", self.kind, self.desc));
        match self.detail.as_ref() {
            Some(ref det) => write!(f, " ({})", det),
            None => Ok(())
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RawMessage<'a> {
    prefix: Option<&'a str>,
    command: &'a str,
    params: &'a str,
}

impl<'a> RawMessage<'a> {
    /// Create a new `RawMessage` with provided parameters
    ///
    /// This function will not validate the parameters
    pub fn new(prefix: Option<&'a str>,
               command: &'a str,
               params: &'a str) -> RawMessage<'a>
    {
        RawMessage {
            prefix: prefix,
            command: command,
            params: params,
        }
    }

    /// Parse the string to a `RawMessage`
    pub fn from_str(s: &'a str) -> Result<RawMessage<'a>, ParseMessageError> {
        let mut cur = s.trim_left();

        let prefix = if cur.starts_with(":") {
            match cur.find(' ') {
                None => {
                    let p = &cur[1..];
                    cur = &cur[cur.len()..];
                    Some(p)
                },
                Some(idx) => {
                    let p = &cur[1..idx];
                    cur = cur[idx..].trim_left();
                    Some(p)
                }
            }
        } else {
            None
        };

        let command = match cur.find(' ') {
            None => {
                let c = cur;
                cur = &cur[cur.len()..];
                c
            },
            Some(idx) => {
                let c = &cur[..idx];
                cur = cur[idx..].trim_left();
                c
            }
        };

        if command.is_empty() {
            return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                              "Require a command or a numeric error code"));
        }

        let msg = RawMessage {
            prefix: prefix,
            command: command,
            params: cur,
        };

        Ok(msg)
    }

    /// Get prefix
    pub fn prefix(&self) -> Option<&'a str> {
        self.prefix
    }

    /// Get command
    pub fn command(&self) -> &'a str {
        &self.command
    }

    /// Get parameters
    pub fn parameters(&self) -> MessageParamIter<'a> {
        MessageParamIter {
            data: &self.params,
        }
    }
}

pub struct MessageParamIter<'a> {
    data: &'a str,
}

impl<'a> MessageParamIter<'a> {
    pub fn wrap(s: &'a str) -> MessageParamIter<'a> {
        MessageParamIter {
            data: s,
        }
    }

    pub fn get(&self) -> &'a str {
        self.data
    }
}

impl<'a> Iterator for MessageParamIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.data.is_empty() {
            None
        } else if self.data.starts_with(":") {
            let cur = &self.data[1..];
            self.data = &self.data[self.data.len()..];
            Some(cur)
        } else {
            match self.data.find(' ') {
                None => {
                    let cur = self.data;
                    self.data = &self.data[self.data.len()..];
                    Some(cur)
                },
                Some(idx) => {
                    let cur = &self.data[..idx];
                    self.data = self.data[idx..].trim_left();
                    Some(cur)
                }
            }
        }
    }
}

impl<'a> fmt::Display for RawMessage<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(prefix) = self.prefix.as_ref() {
            try!(write!(f, ":{} ", prefix))
        }

        try!(write!(f, "{}", self.command));

        if !self.params.is_empty() {
            try!(write!(f, " {}", self.params));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageCommand<'a> {
    Command(Command<'a>),
    Reply(Reply<'a>),
    Unrecognized(RawMessage<'a>),
}

impl<'a> fmt::Display for MessageCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &MessageCommand::Command(ref c) => c.fmt(f),
            &MessageCommand::Reply(ref c) => c.fmt(f),
            &MessageCommand::Unrecognized(ref c) => c.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Message<'a> {
    prefix: Option<&'a str>,
    command: MessageCommand<'a>,
}

impl<'a> Message<'a> {
    pub fn new(prefix: Option<&'a str>, command: MessageCommand<'a>) -> Message<'a> {
        Message {
            prefix: prefix,
            command: command,
        }
    }

    pub fn prefix<'s>(&'s self) -> Option<&'a str> {
        self.prefix
    }

    pub fn command<'s>(&'s self) -> &'s MessageCommand<'a> {
        &self.command
    }
}

impl<'a> fmt::Display for Message<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref prefix) = self.prefix.as_ref() {
            try!(write!(f, "{} ", prefix));
        }

        write!(f, "{}", self.command)
    }
}

impl<'a> IrcMessage<'a> for Message<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<Message<'a>, ParseMessageError> {
        let prefix = raw.prefix();

        let command = {
            match Reply::from_raw(raw) {
                Ok(cmd) => {
                    if prefix.is_none() {
                        return Err(ParseMessageError::new(ParseMessageErrorKind::MissingPrefix,
                                                          "Prefix is required in reply"));
                    }

                    MessageCommand::Reply(cmd)
                },
                Err(err) => match err.kind() {
                    ParseMessageErrorKind::InvalidReplyCode => {
                        // Try to parse Command
                        match Command::from_raw(raw) {
                            Ok(cmd) => MessageCommand::Command(cmd),
                            Err(err) => match err.kind() {
                                ParseMessageErrorKind::UnrecognizedCommand => {
                                    MessageCommand::Unrecognized(*raw)
                                },
                                _ => {
                                    return Err(err);
                                }
                            }
                        }
                    },
                    ParseMessageErrorKind::UnrecognizedReply => {
                        MessageCommand::Unrecognized(*raw)
                    },
                    _ => {
                        return Err(err);
                    }
                }
            }
        };

        Ok(Message::new(prefix, command))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_raw_message_basic() {
        let data = ":fripp.mozilla.org NOTICE Auth :*** Looking up your hostname...";
        let raw_message = RawMessage::from_str(data).unwrap();

        let expected = RawMessage {
            prefix: Some("fripp.mozilla.org"),
            command: "NOTICE",
            params: "Auth :*** Looking up your hostname...",
        };

        assert_eq!(expected, raw_message);
    }

    #[test]
    fn test_raw_message_prefix() {
        let data = ":fripp.mozilla.org NOTICE";
        let raw_message = RawMessage::from_str(data).unwrap();
        assert_eq!(raw_message.prefix(), Some("fripp.mozilla.org"));

        let data = "NOTICE";
        let raw_message = RawMessage::from_str(data).unwrap();
        assert_eq!(raw_message.prefix(), None);
    }

    #[test]
    fn test_raw_message_tailing() {
        let data = r#":fripp.mozilla.org 005 zonyitoo AWAYLEN=200 CASEMAPPING=rfc1459 CHANMODES=Zbeg,k,FLfjl,ABCDKMNOQRSTcimnprstuz :are supported by this server"#;
        let raw_message = RawMessage::from_str(data).unwrap();

        let params: Vec<&str> = raw_message.parameters().collect();
        let expected = [
            "zonyitoo",
            "AWAYLEN=200",
            "CASEMAPPING=rfc1459",
            "CHANMODES=Zbeg,k,FLfjl,ABCDKMNOQRSTcimnprstuz",
            "are supported by this server",
        ];

        assert_eq!(&expected, &params[..]);
    }

    #[test]
    fn test_raw_message_serialize() {
        let message = RawMessage::new(Some("fripp.mozilla.org"), "NOTICE",
                                      "Auth :*** Looking up your hostname...");
        let expected = ":fripp.mozilla.org NOTICE Auth :*** Looking up your hostname...";

        assert_eq!(&message.to_string()[..], expected);
    }

    #[test]
    fn test_message_basic() {
        use protocol::command::{Command, UserCommand};

        let data = "USER guest 0 * :Ronnie Reagan";
        let msg = Message::from_str(data).unwrap();

        let expected = Message::new(None, MessageCommand::Command(Command::User(UserCommand::new("guest", 0,
                                                                                                 "Ronnie Reagan"))));
        assert_eq!(expected, msg);
    }

    #[test]
    fn test_message_basic_2() {
        use protocol::reply::{Reply, ReplyCode};

        let data = ":fripp.mozilla.org 001 zonyitoo :Welcome to the Mozilla IRC Network zonyitoo!zonyitoo@113.93.181.139";
        let msg = Message::from_str(data).unwrap();

        let expected = Message::new(Some("fripp.mozilla.org"),
                                    MessageCommand::Reply(Reply::new(ReplyCode::RPL_WELCOME,
                                                                     "zonyitoo",
                                                                     ":Welcome to the Mozilla IRC Network zonyitoo!zonyitoo@113.93.181.139")));
        assert_eq!(expected, msg);
    }
}
