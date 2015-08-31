use std::fmt;
use std::convert::From;

use protocol::message::{IrcMessage, RawMessage, ParseMessageError, ParseMessageErrorKind};

pub use self::pass::PassCommand;
pub use self::nick::NickCommand;
pub use self::user::UserCommand;
pub use self::notice::NoticeCommand;
pub use self::privmsg::PrivmsgCommand;
pub use self::join::JoinCommand;
pub use self::quit::QuitCommand;
pub use self::ping::PingCommand;
pub use self::pong::PongCommand;

pub mod pass;
pub mod nick;
pub mod user;
pub mod notice;
pub mod privmsg;
pub mod join;
pub mod quit;
pub mod ping;
pub mod pong;

// Connection Registration
pub const CMD_PASS: &'static str = "PASS";
pub const CMD_NICK: &'static str = "NICK";
pub const CMD_USER: &'static str = "USER";
pub const CMD_OPER: &'static str = "OPER";
pub const CMD_MODE: &'static str = "MODE";
pub const CMD_SERVICE: &'static str = "SERVICE";
pub const CMD_QUIT: &'static str = "QUIT";
pub const CMD_SQUIT: &'static str = "SQUIT";

// Channel operations
pub const CMD_JOIN: &'static str = "JOIN";
pub const CMD_PART: &'static str = "PART";
pub const CMD_TOPIC: &'static str = "TOPIC";
pub const CMD_NAMES: &'static str = "NAMES";
pub const CMD_LIST: &'static str = "LIST";
pub const CMD_INVITE: &'static str = "INVITE";
pub const CMD_KICK: &'static str = "KICK";

// Sending messages
pub const CMD_PRIVMSG: &'static str = "PRIVMSG";
pub const CMD_NOTICE: &'static str = "NOTICE";

// Server queries and commands
pub const CMD_MOTD: &'static str = "MOTD";
pub const CMD_LUSERS: &'static str = "LUSERS";
pub const CMD_VERSION: &'static str = "VERSION";
pub const CMD_STATS: &'static str = "STATS";
pub const CMD_LINKS: &'static str = "LINKS";
pub const CMD_TIME: &'static str = "TIME";
pub const CMD_CONNECT: &'static str = "CONNECT";
pub const CMD_TRACE: &'static str = "TRACE";
pub const CMD_ADMIN: &'static str = "ADMIN";
pub const CMD_INFO: &'static str = "INFO";

// Service Query and Commands
pub const CMD_SERVLIST: &'static str = "SERVLIST";
pub const CMD_SQUERY: &'static str = "SQUERY";

// User based queries
pub const CMD_WHO: &'static str = "WHO";
pub const CMD_WHOIS: &'static str = "WHOIS";
pub const CMD_WHOWAS: &'static str = "WHOWAS";

// Miscellaneous messages
pub const CMD_KILL: &'static str = "KILL";
pub const CMD_PING: &'static str = "PING";
pub const CMD_PONG: &'static str = "PONG";
pub const CMD_ERROR: &'static str = "ERROR";

// Optional features
pub const CMD_AWAY: &'static str = "AWAY";
pub const CMD_REHASH: &'static str = "REHASH";
pub const CMD_DIE: &'static str = "DIE";
pub const CMD_RESTART: &'static str = "RESTART";
pub const CMD_SUMMON: &'static str = "SUMMON";
pub const CMD_USERS: &'static str = "USERS";
pub const CMD_WALLOPS: &'static str = "WALLOPS";
pub const CMD_USERHOST: &'static str = "USERHOST";
pub const CMD_ISON: &'static str = "ISON";


impl<'a> Command<'a> {
    pub fn new<C>(c: C) -> Command<'a>
        where Command<'a>: From<C>
    {
        From::from(c)
    }
}

macro_rules! impl_cmd_from {
    ($name:ident, $structname:ty) => {
        impl<'a> From<$structname> for Command<'a> {
            fn from(n: $structname) -> Command<'a> {
                Command::$name(n)
            }
        }
    }
}

macro_rules! impl_cmd {
    ($($cmd:ident # $name:ident => $sname:ty,)+) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        pub enum Command<'a> {
            $(
                $name($sname),
            )+
        }

        impl<'a> IrcMessage<'a> for Command<'a> {
            fn from_raw(raw: &RawMessage<'a>) -> Result<Command<'a>, ParseMessageError> {
                match raw.command() {
                    $(
                        $cmd => Ok(Command::$name(try!(IrcMessage::from_raw(raw)))),
                    )+

                    _ => Err(ParseMessageError::new(ParseMessageErrorKind::UnrecognizedCommand,
                                                    "Unrecognized command")),
                }
            }
        }

        impl<'a> fmt::Display for Command<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(
                        &Command::$name(ref c) => c.fmt(f),
                    )+
                }
            }
        }

        $(
            impl_cmd_from!($name, $sname);
        )+
    }
}


impl_cmd! {
    CMD_PASS    # Pass      => PassCommand<'a>,
    CMD_NICK    # Nick      => NickCommand<'a>,
    CMD_USER    # User      => UserCommand<'a>,
    CMD_JOIN    # Join      => JoinCommand<'a>,
    CMD_PRIVMSG # Privmsg   => PrivmsgCommand<'a>,
    CMD_NOTICE  # Notice    => NoticeCommand<'a>,
    CMD_QUIT    # Quit      => QuitCommand<'a>,
    CMD_PING    # Ping      => PingCommand<'a>,
    CMD_PONG    # Pong      => PongCommand<'a>,
}
