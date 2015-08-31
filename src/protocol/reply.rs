
use std::fmt;

use protocol::message::{RawMessage, IrcMessage, MessageParamIter, ParseMessageError, ParseMessageErrorKind};

/// Numerics in the range from 001 to 099 are used for client-server
/// connections only and should never travel between servers.  Replies
/// generated in the response to commands are found in the range from 200
/// to 399.
#[repr(u16)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ReplyCode {
    /// The server sends Replies 001 to 004 to a user upon
    /// successful registration.
    RPL_WELCOME         = 1,
    RPL_YOURHOST        = 2,
    RPL_CREATED         = 3,
    RPL_MYINFO          = 4,
    /// Sent by the server to a user to suggest an alternative
    /// server.  This is often used when the connection is
    /// refused because the server is already full.
    RPL_BOUNCE          = 5,

    /// Reply format used by `USERHOST` to list replies to
    /// the query list.  The reply string is composed as
    /// follows:
    ///
    /// reply = nickname [ "*" ] "=" ( "+" / "-" ) hostname
    ///
    /// The '*' indicates whether the client has registered
    /// as an Operator.  The '-' or '+' characters represent
    /// whether the client has set an `AWAY` message or not
    /// respectively.
    RPL_USERHOST        = 302,

    /// Reply format used by `ISON` to list replies to the
    /// query list.
    RPL_ISON            = 303,

    /// These replies are used with the `AWAY` command (if
    /// allowed).  `RPL_AWAY` is sent to any client sending a
    /// `PRIVMSG` to a client which is away.  `RPL_AWAY` is only
    /// sent by the server to which the client is connected.
    /// Replies `RPL_UNAWAY` and `RPL_NOWAWAY` are sent when the
    /// client removes and sets an `AWAY `message.
    RPL_AWAY            = 301,
    RPL_UNAWAY          = 305,
    RPL_NOWAWAY         = 306,

    /// Replies 311 - 313, 317 - 319 are all replies
    /// generated in response to a `WHOIS` message.  Given that
    /// there are enough parameters present, the answering
    /// server MUST either formulate a reply out of the above
    /// numerics (if the query nick is found) or return an
    /// error reply.  The '*' in `RPL_WHOISUSER` is there as
    /// the literal character and not as a wild card.  For
    /// each reply set, only `RPL_WHOISCHANNELS` may appear
    /// more than once (for long lists of channel names).
    /// The '@' and '+' characters next to the channel name
    /// indicate whether a client is a channel operator or
    /// has been granted permission to speak on a moderated
    /// channel.  The `RPL_ENDOFWHOIS` reply is used to mark
    /// the end of processing a `WHOIS` message.
    RPL_WHOISUSER       = 311,
    RPL_WHOISSERVER     = 312,
    RPL_WHOISOPERATOR   = 313,
    RPL_WHOISIDLE       = 317,
    RPL_ENDOFWHOIS      = 318,
    RPL_WHOISCHANNELS   = 319,

    /// When replying to a `WHOWAS` message, a server MUST use
    /// the replies `RPL_WHOWASUSER`, `RPL_WHOISSERVER` or
    /// `ERR_WASNOSUCHNICK` for each nickname in the presented
    /// list.  At the end of all reply batches, there MUST
    /// be `RPL_ENDOFWHOWAS` (even if there was only one reply
    /// and it was an error).
    RPL_WHOWASUSER      = 314,
    RPL_ENDOFWHOWAS     = 369,

    /// Obsolete
    RPL_LISTSTART       = 321,

    /// Replies `RPL_LIST`, `RPL_LISTEND` mark the actual replies
    /// with data and end of the server's response to a `LIST`
    /// command.  If there are no channels available to return,
    /// only the end reply MUST be sent.
    RPL_LIST            = 322,
    RPL_LISTEND         = 323,

    /// When sending a `TOPIC` message to determine the
    /// channel topic, one of two replies is sent.  If
    /// the topic is set, `RPL_TOPIC` is sent back else
    /// `RPL_NOTOPIC`.
    RPL_UNIQOPIS        = 325,
    RPL_CHANNELMODEIS   = 324,
    RPL_NOTOPIC         = 331,
    RPL_TOPIC           = 332,

    /// Returned by the server to indicate that the
    /// attempted `INVITE` message was successful and is
    /// being passed onto the end client.
    RPL_INVITING        = 341,

    /// Returned by a server answering a `SUMMON` message to
    /// indicate that it is summoning that user.
    RPL_SUMMONING       = 342,

    /// When listing the 'invitations masks' for a given channel,
    /// a server is required to send the list back using the
    /// `RPL_INVITELIST` and `RPL_ENDOFINVITELIST` messages.  A
    /// separate `RPL_INVITELIST` is sent for each active mask.
    /// After the masks have been listed (or if none present) a
    /// `RPL_ENDOFINVITELIST` MUST be sent.
    RPL_INVITELIST      = 346,
    RPL_ENDOFINVITELIST = 347,

    /// When listing the 'exception masks' for a given channel,
    /// a server is required to send the list back using the
    /// `RPL_EXCEPTLIST` and `RPL_ENDOFEXCEPTLIST` messages.  A
    /// separate `RPL_EXCEPTLIST` is sent for each active mask.
    /// After the masks have been listed (or if none present)
    /// a `RPL_ENDOFEXCEPTLIST` MUST be sent.
    RPL_EXCEPTLIST      = 348,
    RPL_ENDOFEXCEPTLIST = 349,

    /// Reply by the server showing its version details.
    /// The `<version>` is the version of the software being
    /// used (including any patchlevel revisions) and the
    /// `<debuglevel>`is used to indicate if the server is
    /// running in "debug mode".
    ///
    /// The "comments" field may contain any comments about
    /// the version or further version details.
    RPL_VERSION         = 351,

    /// The `RPL_WHOREPLY` and `RPL_ENDOFWHO` pair are used
    /// to answer a `WHO` message.  The `RPL_WHOREPLY` is only
    /// sent if there is an appropriate match to the WHO
    /// query.  If there is a list of parameters supplied
    /// with a `WHO` message, a `RPL_ENDOFWHO` MUST be sent
    /// after processing each list item with `<name>` being
    /// the item.
    RPL_WHOREPLY        = 352,
    RPL_ENDOFWHO        = 315,

    /// To reply to a `NAMES` message, a reply pair consisting
    /// of `RPL_NAMREPLY` and `RPL_ENDOFNAMES` is sent by the
    /// server back to the client.  If there is no channel
    /// found as in the query, then only `RPL_ENDOFNAMES` is
    /// returned.  The exception to this is when a NAMES
    /// message is sent with no parameters and all visible
    /// channels and contents are sent back in a series of
    /// `RPL_NAMEREPLY` messages with a `RPL_ENDOFNAMES` to mark
    /// the end.
    RPL_NAMREPLY        = 353,
    RPL_ENDOFNAMES      = 366,

    /// In replying to the `LINKS` message, a server MUST send
    /// replies back using the `RPL_LINKS` numeric and mark the
    /// end of the list using an `RPL_ENDOFLINKS` reply.
    RPL_LINKS           = 364,
    RPL_ENDOFLINKS      = 365,

    /// When listing the active 'bans' for a given channel,
    /// a server is required to send the list back using the
    /// `RPL_BANLIST` and `RPL_ENDOFBANLIST` messages.  A separate
    /// `RPL_BANLIST` is sent for each active banmask.  After the
    /// banmasks have been listed (or if none present) a
    /// `RPL_ENDOFBANLIST` MUST be sent.
    RPL_BANLIST         = 367,
    RPL_ENDOFBANLIST    = 368,

    /// A server responding to an `INFO` message is required to
    /// send all its 'info' in a series of `RPL_INFO` messages
    /// with a `RPL_ENDOFINFO` reply to indicate the end of the
    /// replies.
    RPL_INFO            = 371,
    RPL_ENDOFINFO       = 374,

    /// When responding to the `MOTD` message and the `MOTD` file
    /// is found, the file is displayed line by line, with
    /// each line no longer than 80 characters, using
    /// `RPL_MOTD` format replies.  These MUST be surrounded
    /// by a `RPL_MOTDSTART` (before the `RPL_MOTD`s) and an
    /// `RPL_ENDOFMOTD` (after).
    RPL_MOTDSTART       = 375,
    RPL_MOTD            = 372,
    RPL_ENDOFMOTD       = 376,

    /// `RPL_YOUREOPER` is sent back to a client which has
    /// just successfully issued an `OPER` message and gained
    /// operator status.
    RPL_YOUREOPER       = 381,

    /// If the `REHASH` option is used and an operator sends
    /// a `REHASH` message, an `RPL_REHASHING` is sent back to
    /// the operator.
    RPL_REHASHING       = 382,

    /// Sent by the server to a service upon successful
    /// registration.
    RPL_YOURESERVICE    = 383,

    /// When replying to the `TIME` message, a server MUST send
    /// the reply using the `RPL_TIME` format above.  The string
    /// showing the time need only contain the correct day and
    /// time there.  There is no further requirement for the
    /// time string.
    RPL_TIME            = 391,

    /// If the `USERS` message is handled by a server, the
    /// replies `RPL_USERSTART`, `RPL_USERS`, `RPL_ENDOFUSERS` and
    /// `RPL_NOUSERS` are used.  `RPL_USERSSTART` MUST be sent
    /// first, following by either a sequence of `RPL_USERS`
    /// or a single `RPL_NOUSER`.  Following this is
    /// `RPL_ENDOFUSERS`.
    RPL_USERSSTART      = 392,
    RPL_USERS           = 393,
    RPL_ENDOFUSERS      = 394,
    RPL_NOUSERS         = 395,

    /// The `RPL_TRACE*` are all returned by the server in
    /// response to the `TRACE` message.  How many are
    /// returned is dependent on the `TRACE` message and
    /// whether it was sent by an operator or not.  There
    /// is no predefined order for which occurs first.
    /// Replies `RPL_TRACEUNKNOWN`, `RPL_TRACECONNECTING` and
    /// `RPL_TRACEHANDSHAKE` are all used for connections
    /// which have not been fully established and are either
    /// unknown, still attempting to connect or in the
    /// process of completing the 'server handshake'.
    /// `RPL_TRACELINK` is sent by any server which handles
    /// a `TRACE` message and has to pass it on to another
    /// server.  The list of `RPL_TRACELINK`s sent in
    /// response to a `TRACE` command traversing the IRC
    /// network should reflect the actual connectivity of
    /// the servers themselves along that path.
    ///
    /// `RPL_TRACENEWTYPE` is to be used for any connection
    /// which does not fit in the other categories but is
    /// being displayed anyway.
    ///
    /// `RPL_TRACEEND` is sent to indicate the end of the list.
    RPL_TRACELINK       = 200,
    RPL_TRACECONNECTING = 201,
    RPL_TRACEHANDSHAKE  = 202,
    RPL_TRACEUNKNOWN    = 203,
    RPL_TRACEOPERATOR   = 204,
    RPL_TRACEUSER       = 205,
    RPL_TRACESERVER     = 206,
    RPL_TRACESERVICE    = 207,
    RPL_TRACENEWTYPE    = 208,
    RPL_TRACECLASS      = 209,
    RPL_TRACERECONNECT  = 210,
    RPL_TRACELOG        = 261,
    RPL_TRACEEND        = 262,

    /// reports statistics on a connection.  `<linkname>`
    /// identifies the particular connection, `<sendq>` is
    /// the amount of data that is queued and waiting to be
    /// sent `<sent messages>` the number of messages sent,
    /// and `<sent Kbytes>` the amount of data sent, in
    /// Kbytes. `<received messages>` and `<received Kbytes>`
    /// are the equivalent of `<sent messages>` and `<sent
    /// Kbytes>` for received data, respectively.  `<time
    /// open>` indicates how long ago the connection was
    /// opened, in seconds.
    RPL_STATSLINKINFO   = 211,

    /// reports statistics on commands usage.
    RPL_STATSCOMMANDS   = 212,

    RPL_ENDOFSTATS      = 219,

    /// reports the server uptime.
    RPL_STATSUPTIME     = 242,

    /// reports the allowed hosts from where user may become IRC
    /// operators.
    RPL_STATSOLINE      = 243,

    /// To answer a query about a client's own mode,
    /// `RPL_UMODEIS` is sent back.
    RPL_UMODEIS         = 221,

    /// When listing services in reply to a `SERVLIST` message,
    /// a server is required to send the list back using the
    /// `RPL_SERVLIST` and `RPL_SERVLISTEND` messages.  A separate
    /// `RPL_SERVLIST` is sent for each service.  After the
    /// services have been listed (or if none present) a
    /// `RPL_SERVLISTEND` MUST be sent.
    RPL_SERVLIST        = 234,
    RPL_SERVLISTEND     = 235,

    /// In processing an `LUSERS` message, the server
    /// sends a set of replies from `RPL_LUSERCLIENT`,
    /// `RPL_LUSEROP`, `RPL_USERUNKNOWN`,
    /// `RPL_LUSERCHANNELS` and `RPL_LUSERME`.  When
    /// replying, a server MUST send back
    /// `RPL_LUSERCLIENT` and `RPL_LUSERME`.  The other
    /// replies are only sent back if a non-zero count
    /// is found for them.
    RPL_LUSERCLIENT     = 251,
    RPL_LUSEROP         = 252,
    RPL_LUSERUNKNOWN    = 253,
    RPL_LUSERCHANNELS   = 254,
    RPL_LUSERME         = 255,

    /// When replying to an `ADMIN` message, a server
    /// is expected to use replies `RPL_ADMINME`
    /// through to `RPL_ADMINEMAIL` and provide a text
    /// message with each.  For `RPL_ADMINLOC1` a
    /// description of what city, state and country
    /// the server is in is expected, followed by
    /// details of the institution (`RPL_ADMINLOC2`)
    /// and finally the administrative contact for the
    /// server (an email address here is REQUIRED)
    /// in `RPL_ADMINEMAIL.
    RPL_ADMINME         = 256,
    RPL_ADMINLOC1       = 257,
    RPL_ADMINLOC2       = 258,
    RPL_ADMINEMAIL      = 259,

    /// When a server drops a command without processing it,
    /// it MUST use the reply `RPL_TRYAGAIN` to inform the
    /// originating client.
    RPL_TRYAGAIN        = 263,

    // Errors

    /// Used to indicate the nickname parameter supplied to a
    /// command is currently unused.
    ERR_NOSUCHNICK      = 401,

    /// Used to indicate the server name given currently
    /// does not exist.
    ERR_NOSUCHSERVER    = 402,

    /// Used to indicate the given channel name is invalid.
    ERR_NOSUCHCHANNEL   = 403,

    /// Sent to a user who is either (a) not on a channel
    /// which is mode `+n` or (b) not a chanop (or mode `+v`) on
    /// a channel which has mode `+m` set or where the user is
    /// banned and is trying to send a `PRIVMSG` message to
    /// that channel.
    ERR_CANNOTSENDTOCHAN    = 404,

    /// Sent to a user when they have joined the maximum
    /// number of allowed channels and they try to join
    /// another channel.
    ERR_TOOMANYCHANNELS = 405,

    /// Returned by `WHOWAS` to indicate there is no history
    /// information for that nickname.
    ERR_WASNOSUCHNICK   = 406,

    /// - Returned to a client which is attempting to send a
    ///   `PRIVMSG`/`NOTICE` using the `user@host` destination format
    ///   and for a `user@host` which has several occurrences.
    ///
    /// - Returned to a client which trying to send a
    ///   `PRIVMSG`/`NOTICE` to too many recipients.
    ///
    /// - Returned to a client which is attempting to `JOIN` a safe
    ///   channel using the shortname when there are more than one
    ///   such channel.
    ERR_TOOMANYTARGETS  = 407,

    /// Returned to a client which is attempting to send a `SQUERY`
    /// to a service which does not exist.
    ERR_NOSUCHSERVICE   = 408,

    /// `PING` or `PONG` message missing the originator parameter.
    ERR_NOORIGIN        = 409,

    /// 412 - 415 are returned by `PRIVMSG` to indicate that
    /// the message wasn't delivered for some reason.
    /// `ERR_NOTOPLEVEL` and `ERR_WILDTOPLEVEL` are errors that
    /// are returned when an invalid use of
    /// `PRIVMSG $<server>` or `PRIVMSG #<host>` is attempted.
    ERR_NORECIPIENT     = 411,
    ERR_NOTEXTTOSEND    = 412,
    ERR_NOTOPLEVEL      = 413,
    ERR_WILDTOPLEVEL    = 414,
    ERR_BADMASK         = 415,

    /// Returned to a registered client to indicate that the
    /// command sent is unknown by the server.
    ERR_UNKNOWNCOMMAND  = 421,

    /// Server's `MOTD` file could not be opened by the server.
    ERR_NOMOTD          = 422,

    /// Returned by a server in response to an `ADMIN` message
    /// when there is an error in finding the appropriate
    /// information.
    ERR_NOADMININFO     = 423,

    /// Generic error message used to report a failed file
    /// operation during the processing of a message.
    ERR_FILEERROR       = 424,

    /// Returned when a nickname parameter expected for a
    /// command and isn't found.
    ERR_NONICKNAMEGIVEN = 431,

    /// Returned after receiving a `NICK` message which contains
    /// characters which do not fall in the defined set.
    ERR_ERRONEUSNICKNAME    = 432,

    /// Returned when a `NICK` message is processed that results
    /// in an attempt to change to a currently existing
    /// nickname.
    ERR_NICKNAMEINUSE   = 433,

    /// Returned by a server to a client when it detects a
    /// nickname collision (registered of a `NICK` that
    /// already exists by another server).
    ERR_NICKCOLLISION   = 436,

    /// - Returned by a server to a user trying to join a channel
    ///   currently blocked by the channel delay mechanism.
    ///
    /// - Returned by a server to a user trying to change nickname
    ///   when the desired nickname is blocked by the nick delay
    ///   mechanism.
    ERR_UNAVAILRESOURCE = 437,

    /// Returned by the server to indicate that the target
    /// user of the command is not on the given channel.
    ERR_USERNOTINCHANNEL    = 441,

    /// Returned by the server whenever a client tries to
    /// perform a channel affecting command for which the
    /// client isn't a member.
    ERR_NOTONCHANNEL    = 442,

    /// Returned when a client tries to invite a user to a
    /// channel they are already on.
    ERR_USERONCHANNEL   = 443,

    /// Returned by the summon after a `SUMMON` command for a
    /// user was unable to be performed since they were not
    /// logged in.
    ERR_NOLOGIN         = 444,

    /// Returned as a response to the `SUMMON` command.  MUST be
    /// returned by any server which doesn't implement it.
    ERR_SUMMONDISABLED  = 445,

    /// Returned as a response to the `USERS` command.  MUST be
    /// returned by any server which does not implement it.
    ERR_USERSDISABLED   = 446,

    /// Returned by the server to indicate that the client
    /// MUST be registered before the server will allow it
    /// to be parsed in detail.
    ERR_NOTREGISTERED   = 451,

    /// Returned by the server by numerous commands to
    /// indicate to the client that it didn't supply enough
    /// parameters.
    ERR_NEEDMOREPARAMS  = 461,

    /// Returned by the server to any link which tries to
    /// change part of the registered details (such as
    /// password or user details from second `USER` message).
    ERR_ALREADYREGISTRED    = 462,

    /// Returned to a client which attempts to register with
    /// a server which does not been setup to allow
    /// connections from the host the attempted connection
    /// is tried.
    ERR_NOPERMFORHOST   = 463,

    /// Returned to indicate a failed attempt at registering
    /// a connection for which a password was required and
    /// was either not given or incorrect.
    ERR_PASSWDMISMATCH  = 464,

    /// Returned after an attempt to connect and register
    /// yourself with a server which has been setup to
    /// explicitly deny connections to you.
    ERR_YOUREBANNEDCREEP    = 465,

    /// Sent by a server to a user to inform that access to the
    /// server will soon be denied.
    ERR_YOUWILLBEBANNED = 466,

    ERR_KEYSET          = 467,
    ERR_CHANNELISFULL   = 471,
    ERR_UNKNOWNMODE     = 472,
    ERR_INVITEONLYCHAN  = 473,
    ERR_BANNEDFROMCHAN  = 474,
    ERR_BADCHANNELKEY   = 475,
    ERR_BADCHANMASK     = 476,
    ERR_NOCHANMODES     = 477,
    ERR_BANLISTFULL     = 478,

    /// Any command requiring operator privileges to operate
    /// MUST return this error to indicate the attempt was
    /// unsuccessful.
    ERR_NOPRIVILEGES    = 481,

    /// Any command requiring 'chanop' privileges (such as
    /// `MODE` messages) MUST return this error if the client
    /// making the attempt is not a chanop on the specified
    /// channel.
    ERR_CHANOPRIVSNEEDED    = 482,

    /// Any attempts to use the `KILL` command on a server
    /// are to be refused and this error returned directly
    /// to the client.
    ERR_CANTKILLSERVER  = 483,

    /// Sent by the server to a user upon connection to indicate
    /// the restricted nature of the connection (user mode `+r`).
    ERR_RESTRICTED      = 484,

    /// Any `MODE` requiring "channel creator" privileges MUST
    /// return this error if the client making the attempt is not
    /// a chanop on the specified channel.
    ERR_UNIQOPPRIVSNEEDED   = 485,

    /// If a client sends an `OPER` message and the server has
    /// not been configured to allow connections from the
    /// client's host as an operator, this error MUST be
    /// returned.
    ERR_NOOPERHOST      = 491,

    /// Returned by the server to indicate that a `MODE`
    /// message was sent with a nickname parameter and that
    /// the a mode flag sent was not recognized.
    ERR_UMODEUNKNOWNFLAG    = 501,

    /// Error sent to any user trying to view or change the
    /// user mode for a user other than themselves.
    ERR_USERSDONTMATCH  = 502,
}

impl ReplyCode {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    pub fn from_u16(c: u16) -> Option<ReplyCode> {
        match c {
            1   => Some(ReplyCode::RPL_WELCOME),
            2   => Some(ReplyCode::RPL_YOURHOST),
            3   => Some(ReplyCode::RPL_CREATED),
            4   => Some(ReplyCode::RPL_MYINFO),
            5   => Some(ReplyCode::RPL_BOUNCE),
            302 => Some(ReplyCode::RPL_USERHOST),
            303 => Some(ReplyCode::RPL_ISON),
            301 => Some(ReplyCode::RPL_AWAY),
            305 => Some(ReplyCode::RPL_UNAWAY),
            306 => Some(ReplyCode::RPL_NOWAWAY),
            311 => Some(ReplyCode::RPL_WHOISUSER),
            312 => Some(ReplyCode::RPL_WHOISSERVER),
            313 => Some(ReplyCode::RPL_WHOISOPERATOR),
            317 => Some(ReplyCode::RPL_WHOISIDLE),
            318 => Some(ReplyCode::RPL_ENDOFWHOIS),
            319 => Some(ReplyCode::RPL_WHOISCHANNELS),
            314 => Some(ReplyCode::RPL_WHOWASUSER),
            369 => Some(ReplyCode::RPL_ENDOFWHOWAS),
            321 => Some(ReplyCode::RPL_LISTSTART),
            322 => Some(ReplyCode::RPL_LIST),
            323 => Some(ReplyCode::RPL_LISTEND),
            325 => Some(ReplyCode::RPL_UNIQOPIS),
            324 => Some(ReplyCode::RPL_CHANNELMODEIS),
            331 => Some(ReplyCode::RPL_NOTOPIC),
            332 => Some(ReplyCode::RPL_TOPIC),
            341 => Some(ReplyCode::RPL_INVITING),
            342 => Some(ReplyCode::RPL_SUMMONING),
            346 => Some(ReplyCode::RPL_INVITELIST),
            347 => Some(ReplyCode::RPL_ENDOFINVITELIST),
            348 => Some(ReplyCode::RPL_EXCEPTLIST),
            349 => Some(ReplyCode::RPL_ENDOFEXCEPTLIST),
            351 => Some(ReplyCode::RPL_VERSION),
            352 => Some(ReplyCode::RPL_WHOREPLY),
            315 => Some(ReplyCode::RPL_ENDOFWHO),
            353 => Some(ReplyCode::RPL_NAMREPLY),
            366 => Some(ReplyCode::RPL_ENDOFNAMES),
            364 => Some(ReplyCode::RPL_LINKS),
            365 => Some(ReplyCode::RPL_ENDOFLINKS),
            367 => Some(ReplyCode::RPL_BANLIST),
            368 => Some(ReplyCode::RPL_ENDOFBANLIST),
            371 => Some(ReplyCode::RPL_INFO),
            374 => Some(ReplyCode::RPL_ENDOFINFO),
            375 => Some(ReplyCode::RPL_MOTDSTART),
            372 => Some(ReplyCode::RPL_MOTD),
            376 => Some(ReplyCode::RPL_ENDOFMOTD),
            381 => Some(ReplyCode::RPL_YOUREOPER),
            382 => Some(ReplyCode::RPL_REHASHING),
            383 => Some(ReplyCode::RPL_YOURESERVICE),
            391 => Some(ReplyCode::RPL_TIME),
            392 => Some(ReplyCode::RPL_USERSSTART),
            393 => Some(ReplyCode::RPL_USERS),
            394 => Some(ReplyCode::RPL_ENDOFUSERS),
            395 => Some(ReplyCode::RPL_NOUSERS),
            200 => Some(ReplyCode::RPL_TRACELINK),
            201 => Some(ReplyCode::RPL_TRACECONNECTING),
            202 => Some(ReplyCode::RPL_TRACEHANDSHAKE),
            203 => Some(ReplyCode::RPL_TRACEUNKNOWN),
            204 => Some(ReplyCode::RPL_TRACEOPERATOR),
            205 => Some(ReplyCode::RPL_TRACEUSER),
            206 => Some(ReplyCode::RPL_TRACESERVER),
            207 => Some(ReplyCode::RPL_TRACESERVICE),
            208 => Some(ReplyCode::RPL_TRACENEWTYPE),
            209 => Some(ReplyCode::RPL_TRACECLASS),
            210 => Some(ReplyCode::RPL_TRACERECONNECT),
            261 => Some(ReplyCode::RPL_TRACELOG),
            262 => Some(ReplyCode::RPL_TRACEEND),
            211 => Some(ReplyCode::RPL_STATSLINKINFO),
            212 => Some(ReplyCode::RPL_STATSCOMMANDS),
            219 => Some(ReplyCode::RPL_ENDOFSTATS),
            242 => Some(ReplyCode::RPL_STATSUPTIME),
            243 => Some(ReplyCode::RPL_STATSOLINE),
            221 => Some(ReplyCode::RPL_UMODEIS),
            234 => Some(ReplyCode::RPL_SERVLIST),
            235 => Some(ReplyCode::RPL_SERVLISTEND),
            251 => Some(ReplyCode::RPL_LUSERCLIENT),
            252 => Some(ReplyCode::RPL_LUSEROP),
            253 => Some(ReplyCode::RPL_LUSERUNKNOWN),
            254 => Some(ReplyCode::RPL_LUSERCHANNELS),
            255 => Some(ReplyCode::RPL_LUSERME),
            256 => Some(ReplyCode::RPL_ADMINME),
            257 => Some(ReplyCode::RPL_ADMINLOC1),
            258 => Some(ReplyCode::RPL_ADMINLOC2),
            259 => Some(ReplyCode::RPL_ADMINEMAIL),
            263 => Some(ReplyCode::RPL_TRYAGAIN),

            401 => Some(ReplyCode::ERR_NOSUCHNICK),
            402 => Some(ReplyCode::ERR_NOSUCHSERVER),
            403 => Some(ReplyCode::ERR_NOSUCHCHANNEL),
            404 => Some(ReplyCode::ERR_CANNOTSENDTOCHAN),
            405 => Some(ReplyCode::ERR_TOOMANYCHANNELS),
            406 => Some(ReplyCode::ERR_WASNOSUCHNICK),
            407 => Some(ReplyCode::ERR_TOOMANYTARGETS),
            408 => Some(ReplyCode::ERR_NOSUCHSERVICE),
            409 => Some(ReplyCode::ERR_NOORIGIN),
            411 => Some(ReplyCode::ERR_NORECIPIENT),
            412 => Some(ReplyCode::ERR_NOTEXTTOSEND),
            413 => Some(ReplyCode::ERR_NOTOPLEVEL),
            414 => Some(ReplyCode::ERR_WILDTOPLEVEL),
            415 => Some(ReplyCode::ERR_BADMASK),
            421 => Some(ReplyCode::ERR_UNKNOWNCOMMAND),
            422 => Some(ReplyCode::ERR_NOMOTD),
            423 => Some(ReplyCode::ERR_NOADMININFO),
            424 => Some(ReplyCode::ERR_FILEERROR),
            431 => Some(ReplyCode::ERR_NONICKNAMEGIVEN),
            432 => Some(ReplyCode::ERR_ERRONEUSNICKNAME),
            433 => Some(ReplyCode::ERR_NICKNAMEINUSE),
            436 => Some(ReplyCode::ERR_NICKCOLLISION),
            437 => Some(ReplyCode::ERR_UNAVAILRESOURCE),
            441 => Some(ReplyCode::ERR_USERNOTINCHANNEL),
            442 => Some(ReplyCode::ERR_NOTONCHANNEL),
            443 => Some(ReplyCode::ERR_USERONCHANNEL),
            444 => Some(ReplyCode::ERR_NOLOGIN),
            445 => Some(ReplyCode::ERR_SUMMONDISABLED),
            446 => Some(ReplyCode::ERR_USERSDISABLED),
            451 => Some(ReplyCode::ERR_NOTREGISTERED),
            461 => Some(ReplyCode::ERR_NEEDMOREPARAMS),
            462 => Some(ReplyCode::ERR_ALREADYREGISTRED),
            463 => Some(ReplyCode::ERR_NOPERMFORHOST),
            464 => Some(ReplyCode::ERR_PASSWDMISMATCH),
            465 => Some(ReplyCode::ERR_YOUREBANNEDCREEP),
            466 => Some(ReplyCode::ERR_YOUWILLBEBANNED),
            467 => Some(ReplyCode::ERR_KEYSET),
            471 => Some(ReplyCode::ERR_CHANNELISFULL),
            472 => Some(ReplyCode::ERR_UNKNOWNMODE),
            473 => Some(ReplyCode::ERR_INVITEONLYCHAN),
            474 => Some(ReplyCode::ERR_BANNEDFROMCHAN),
            475 => Some(ReplyCode::ERR_BADCHANNELKEY),
            476 => Some(ReplyCode::ERR_BADCHANMASK),
            477 => Some(ReplyCode::ERR_NOCHANMODES),
            478 => Some(ReplyCode::ERR_BANLISTFULL),
            481 => Some(ReplyCode::ERR_NOPRIVILEGES),
            482 => Some(ReplyCode::ERR_CHANOPRIVSNEEDED),
            483 => Some(ReplyCode::ERR_CANTKILLSERVER),
            484 => Some(ReplyCode::ERR_RESTRICTED),
            485 => Some(ReplyCode::ERR_UNIQOPPRIVSNEEDED),
            491 => Some(ReplyCode::ERR_NOOPERHOST),
            501 => Some(ReplyCode::ERR_UMODEUNKNOWNFLAG),
            502 => Some(ReplyCode::ERR_USERSDONTMATCH),

            _   => None,
        }
    }
}

impl fmt::Display for ReplyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:03}", self.as_u16())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Reply<'a> {
    reply_code: ReplyCode,
    target: &'a str,
    params: &'a str,
}

impl<'a> Reply<'a> {
    pub fn new(code: ReplyCode, target: &'a str, params: &'a str) -> Reply<'a> {
        Reply {
            reply_code: code,
            target: target,
            params: params,
        }
    }

    pub fn code(&self) -> ReplyCode {
        self.reply_code
    }

    pub fn target(&self) -> &'a str {
        self.target
    }

    pub fn parameters(&self) -> MessageParamIter<'a> {
        MessageParamIter::wrap(self.params)
    }
}

impl<'a> fmt::Display for Reply<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {}", self.reply_code, self.target));

        if self.params.is_empty() {
            Ok(())
        } else {
            write!(f, " {}", self.params)
        }
    }
}

impl<'a> IrcMessage<'a> for Reply<'a> {
    fn from_raw(raw: &RawMessage<'a>) -> Result<Reply<'a>, ParseMessageError> {
        let cmd = match raw.command().parse::<u16>() {
            Err(..) => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::InvalidReplyCode,
                                                  "Reply should be a numeric number"))
            },
            Ok(cmd) => cmd,
        };

        let code = match ReplyCode::from_u16(cmd) {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::UnrecognizedReply,
                                                  "Unrecognized reply"));
            },
            Some(code) => code,
        };

        let mut params = raw.parameters();
        let target = match params.next() {
            None => {
                return Err(ParseMessageError::new(ParseMessageErrorKind::NeedMoreParams,
                                                  "Require a target"));
            },
            Some(t) => t,
        };

        Ok(Reply::new(code, target, params.get()))
    }
}
