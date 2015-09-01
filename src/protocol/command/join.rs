use std::fmt;

use protocol::command::*;
use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

pub struct ChannelKeyIter<'a> {
    channels: &'a str,
    keys: Option<&'a str>,
}

impl<'a> Iterator for ChannelKeyIter<'a> {
    type Item = (&'a str, Option<&'a str>);

    fn next(&mut self) -> Option<(&'a str, Option<&'a str>)> {
        if self.channels.is_empty() {
            None
        } else {
            let chan = match self.channels.find(',') {
                None => {
                    let chan = self.channels;
                    self.channels = &self.channels[self.channels.len()..];
                    chan
                },
                Some(idx) => {
                    let chan = &self.channels[..idx];
                    self.channels = &self.channels[idx+1..];
                    chan
                }
            };

            let key = match self.keys.as_mut() {
                None | Some(&mut "") => None,
                Some(k) => {
                    match k.find(',') {
                        None => {
                            let key = *k;
                            *k = &k[k.len()..];
                            Some(key)
                        },
                        Some(idx) => {
                            let key = &k[..idx];
                            *k = &k[idx+1..];
                            Some(key)
                        }
                    }
                }
            };

            Some((chan, key))
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct JoinCommand<'a> {
    channels: &'a str,
    keys: Option<&'a str>,
}

impl<'a> JoinCommand<'a> {
    pub fn new(c: &'a str, keys: Option<&'a str>) -> JoinCommand<'a> {
        JoinCommand {
            channels: c,
            keys: keys,
        }
    }

    pub fn channel_with_keys(&self) -> ChannelKeyIter<'a> {
        ChannelKeyIter {
            channels: self.channels,
            keys: self.keys,
        }
    }
}

impl<'a> fmt::Display for JoinCommand<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", CMD_JOIN, self.channels));

        match self.keys {
            Some(k) => write!(f, "{}", k),
            None => Ok(()),
        }
    }
}

impl<'a> IrcMessage<'a> for JoinCommand<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<JoinCommand<'a>, ParseMessageError> {
        let mut params = raw.parameters();

        let channels = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "JOIN requires channels"));
            },
            Some(n) => n
        };

        let keys = params.next();

        Ok(JoinCommand::new(channels, keys))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use protocol::message::IrcMessage;

    #[test]
    fn test_join_command_basic() {
        let data = "JOIN #foo,#bar fubar,foobar";

        let cmd = JoinCommand::from_str(data).unwrap();

        let expected_chan = [
            ("#foo", Some("fubar")),
            ("#bar", Some("foobar")),
        ];

        let actual: Vec<(&str, Option<&str>)> = cmd.channel_with_keys().collect();

        assert_eq!(&expected_chan[..], &actual[..]);
    }

    #[test]
    fn test_join_command_basic2() {
        let data = "JOIN #foo,#bar fubar";

        let cmd = JoinCommand::from_str(data).unwrap();

        let expected_chan = [
            ("#foo", Some("fubar")),
            ("#bar", None),
        ];

        let actual: Vec<(&str, Option<&str>)> = cmd.channel_with_keys().collect();

        assert_eq!(&expected_chan[..], &actual[..]);
    }

    #[test]
    fn test_join_command_basic3() {
        let data = "JOIN #foo,#bar";

        let cmd = JoinCommand::from_str(data).unwrap();

        let expected_chan = [
            ("#foo", None),
            ("#bar", None),
        ];

        let actual: Vec<(&str, Option<&str>)> = cmd.channel_with_keys().collect();

        assert_eq!(&expected_chan[..], &actual[..]);
    }
}
