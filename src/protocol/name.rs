
use std::fmt;

use regex::Regex;

use protocol::message::{ParseMessageError, ParseMessageErrorKind};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NickName<'a> {
    nick: &'a str,
    user: Option<&'a str>,
    host: Option<&'a str>,
}

impl<'a> NickName<'a> {
    pub fn new(nick: &'a str, user: Option<&'a str>, host: Option<&'a str>) -> NickName<'a> {
        NickName {
            nick: nick,
            user: user,
            host: host,
        }
    }

    pub fn from_str(s: &'a str) -> Result<NickName<'a>, ParseMessageError>  {
        let user_regex = Regex::new(r#"(?x)^
            ([\w-]{1,8})    # Nick
            (![^\r\n @]+)?  # User
            (@(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}
                |(?:[:xdigit:]+(?::[:xdigit:]+){7}|0:0:0:0:0:(?:0|FFFF):\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})
                |[:alnum:][:alnum:-]*[:alnum:]*(?:\.[:alnum:][:alnum:-]*[:alnum:]*)*))?$
        "#).unwrap();

        match user_regex.captures(s) {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::InvalidParam,
                                                  "Invalid name"));
            },
            Some(cap) => {
                let nick: &'a str = cap.at(1).unwrap();
                let user: Option<&'a str> = cap.at(2).map(|u| &u[1..]);
                let host: Option<&'a str> = cap.at(3).map(|h| &h[1..]);

                Ok(NickName::new(nick, user, host))
            }
        }
    }

    pub fn nick(&'a self) -> &'a str {
        &self.nick
    }

    pub fn user(&'a self) -> Option<&'a str> {
        self.user
    }

    pub fn host(&'a self) -> Option<&'a str> {
        self.host
    }
}

impl<'a> fmt::Display for NickName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}", self.nick));

        if let Some(user) = self.user.as_ref() {
            try!(write!(f, "!{}", user));

            if let Some(host) = self.host.as_ref() {
                try!(write!(f, "@{}", host));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_name_basic() {
        let data = "zonyitoo!zonyitoo@123.456.789.10";
        let name = NickName::from_str(data).unwrap();

        assert_eq!(name.nick(), "zonyitoo");
        assert_eq!(name.user(), Some("zonyitoo"));
        assert_eq!(name.host(), Some("123.456.789.10"));
    }

    #[test]
    fn test_name_user_only() {
        let data = "zonyitoo!zonyitoo";
        let name = NickName::from_str(data).unwrap();

        assert_eq!(name.nick(), "zonyitoo");
        assert_eq!(name.user(), Some("zonyitoo"));
        assert_eq!(name.host(), None);
    }

    #[test]
    fn test_name_ipv4_host_only() {
        let data = "zonyitoo@123.456.789.10";
        let name = NickName::from_str(data).unwrap();

        assert_eq!(name.nick(), "zonyitoo");
        assert_eq!(name.user(), None);
        assert_eq!(name.host(), Some("123.456.789.10"));
    }

    #[test]
    fn test_name_nick_only() {
        let data = "zonyitoo";
        let name = NickName::from_str(data).unwrap();

        assert_eq!(name.nick(), "zonyitoo");
        assert_eq!(name.user(), None);
        assert_eq!(name.host(), None);
    }

    #[test]
    fn test_name_ipv6_host_only() {
        let data = "zonyitoo@1111:2222:3333:4444:5555:6666:7777:8888";
        let name = NickName::from_str(data).unwrap();

        assert_eq!(name.nick(), "zonyitoo");
        assert_eq!(name.user(), None);
        assert_eq!(name.host(), Some("1111:2222:3333:4444:5555:6666:7777:8888"));
    }

    #[test]
    fn test_name_name_host_only() {
        let data = "zonyitoo@www.mozilla.org";
        let name = NickName::from_str(data).unwrap();

        assert_eq!(name.nick(), "zonyitoo");
        assert_eq!(name.user(), None);
        assert_eq!(name.host(), Some("www.mozilla.org"));
    }
}
