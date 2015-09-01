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
pub use self::oper::OperCommand;
pub use self::mode::ModeCommand;
pub use self::service::ServiceCommand;
pub use self::squit::SQuitCommand;
pub use self::part::PartCommand;
pub use self::topic::TopicCommand;
pub use self::names::NamesCommand;
pub use self::list::ListCommand;
pub use self::invite::InviteCommand;
pub use self::kick::KickCommand;
pub use self::motd::MotdCommand;
pub use self::version::VersionCommand;
pub use self::lusers::LusersCommand;
pub use self::stats::StatsCommand;
pub use self::links::LinksCommand;
pub use self::time::TimeCommand;
pub use self::connect::ConnectCommand;
pub use self::trace::TraceCommand;
pub use self::admin::AdminCommand;
pub use self::info::InfoCommand;
pub use self::servlist::ServlistCommand;
pub use self::squery::SqueryCommand;
pub use self::error::ErrorCommand;
pub use self::who::WhoCommand;
pub use self::whois::WhoisCommand;
pub use self::away::AwayCommand;
pub use self::rehash::RehashCommand;
pub use self::die::DieCommand;
pub use self::restart::RestartCommand;
pub use self::whowas::WhowasCommand;
pub use self::summon::SummonCommand;
pub use self::users::UsersCommand;
pub use self::wallops::WallopsCommand;
pub use self::userhost::UserhostCommand;
pub use self::ison::IsonCommand;

pub mod pass;
pub mod nick;
pub mod user;
pub mod notice;
pub mod privmsg;
pub mod join;
pub mod quit;
pub mod ping;
pub mod pong;
pub mod oper;
pub mod mode;
pub mod service;
pub mod squit;
pub mod part;
pub mod topic;
pub mod names;
pub mod list;
pub mod invite;
pub mod kick;
pub mod motd;
pub mod version;
pub mod lusers;
pub mod stats;
pub mod links;
pub mod time;
pub mod connect;
pub mod trace;
pub mod admin;
pub mod info;
pub mod servlist;
pub mod squery;
pub mod error;
pub mod who;
pub mod whois;
pub mod whowas;
pub mod away;
pub mod rehash;
pub mod die;
pub mod restart;
pub mod summon;
pub mod users;
pub mod wallops;
pub mod userhost;
pub mod ison;

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
    CMD_OPER    # Oper      => OperCommand<'a>,
    CMD_MODE    # Mode      => ModeCommand<'a>,
    CMD_SERVICE # Service   => ServiceCommand<'a>,
    CMD_QUIT    # Quit      => QuitCommand<'a>,
    CMD_SQUIT   # SQuit     => SQuitCommand<'a>,
    CMD_JOIN    # Join      => JoinCommand<'a>,
    CMD_PART    # Part      => PartCommand<'a>,
    CMD_TOPIC   # Topic     => TopicCommand<'a>,
    CMD_NAMES   # Names     => NamesCommand<'a>,
    CMD_LIST    # List      => ListCommand<'a>,
    CMD_KICK    # Kick      => KickCommand<'a>,
    CMD_PRIVMSG # Privmsg   => PrivmsgCommand<'a>,
    CMD_NOTICE  # Notice    => NoticeCommand<'a>,
    CMD_MOTD    # Motd      => MotdCommand<'a>,
    CMD_LUSERS  # Lusers    => LusersCommand<'a>,
    CMD_VERSION # Version   => VersionCommand<'a>,
    CMD_STATS   # Stats     => StatsCommand<'a>,
    CMD_LINKS   # Links     => LinksCommand<'a>,
    CMD_CONNECT # Connect   => ConnectCommand<'a>,
    CMD_TRACE   # Trace     => TraceCommand<'a>,
    CMD_ADMIN   # Admin     => AdminCommand<'a>,
    CMD_SQUERY  # Squery    => SqueryCommand<'a>,
    CMD_PING    # Ping      => PingCommand<'a>,
    CMD_PONG    # Pong      => PongCommand<'a>,
    CMD_ERROR   # Error     => ErrorCommand<'a>,
    CMD_WHO     # Who       => WhoCommand<'a>,
    CMD_WHOIS   # Whois     => WhoisCommand<'a>,
    CMD_AWAY    # Away      => AwayCommand<'a>,
    CMD_REHASH  # Rehash    => RehashCommand,
    CMD_DIE     # Die       => DieCommand,
    CMD_RESTART # Restart   => RestartCommand,
    CMD_SUMMON  # Summon    => SummonCommand<'a>,
    CMD_WALLOPS # Wallops   => WallopsCommand<'a>,
    CMD_USERHOST # UserHost => UserhostCommand<'a>,
    CMD_ISON    # Ison      => IsonCommand<'a>,
}

pub struct MultipleFieldIter<'a> {
    data: &'a str,
}

impl<'a> MultipleFieldIter<'a> {
    pub fn wrap(d: &'a str) -> MultipleFieldIter<'a> {
        MultipleFieldIter {
            data: d,
        }
    }
}

impl<'a> Iterator for MultipleFieldIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.data.is_empty() {
            None
        } else {
            match self.data.find(',') {
                None => {
                    let cur = self.data;
                    self.data = &self.data[self.data.len()..];
                    Some(cur)
                },
                Some(idx) => {
                    let cur = &self.data[..idx];
                    self.data = &self.data[idx+1..];
                    Some(cur)
                }
            }
        }
    }
}
