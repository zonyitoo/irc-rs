#[macro_use]
extern crate log;
extern crate env_logger;
extern crate irc;

use std::net::TcpStream;
use std::io::{self, Write, BufRead, BufReader};
use std::thread;
use std::env;

use irc::protocol::message::{IrcMessage, Message, Body};
use irc::protocol::command::{Command, UserCommand, NickCommand, JoinCommand, PongCommand, PrivmsgCommand, QuitCommand};
use irc::protocol::reply::ReplyCode;

fn main() {
    env_logger::init().unwrap();

    let username: String = env::args().skip(1).next().expect("You have to provide a username").clone();
    let server_addr = "irc.mozilla.org:6667";

    println!("Logging to {:?} as {:?}", server_addr, username);

    info!("Connecting to {:?}", server_addr);
    let mut stream = TcpStream::connect(server_addr).unwrap();
    let bufreader = BufReader::new(stream.try_clone().unwrap());

    let nickname = &username[..];

    {
        let usercmd = UserCommand::new(&username[..], 0, &username[..]);
        let nickcmd = NickCommand::new(nickname);
        let usermsg = Message::new(None, Body::command(usercmd));
        let nickmsg = Message::new(None, Body::command(nickcmd));

        info!("Authorizing: sending {:?}", usermsg);
        write!(stream, "{}\r\n", usermsg).unwrap();

        info!("Authorizing: sending {:?}", nickmsg);
        write!(stream, "{}\r\n", nickmsg).unwrap();
    }

    let mut lines = bufreader.lines();

    {
        // Waiting for MOTD_START
        while let Some(line) = lines.next() {
            let line = line.unwrap();

            let parsed = Message::from_str(line.trim()).unwrap();
            trace!("{:?}", parsed);

            if let &Body::Reply(ref rpl) = parsed.body() {
                if let ReplyCode::RPL_MOTDSTART = rpl.code() {
                    let params = rpl.parameters().collect::<Vec<&str>>();
                    println!("-*- {}", params.connect(" "));
                    break;
                } else if let ReplyCode::ERR_NICKNAMEINUSE = rpl.code() {
                    panic!("Nickname {:?} already in use", nickname);
                }
            } else if let &Body::Command(ref cmd) = parsed.body() {
                if let &Command::Notice(ref notice) = cmd {
                    println!("-*- Notice: {}", notice.message());
                }
            }
        }

        // Printing MOTD
        while let Some(line) = lines.next() {
            let line = line.unwrap();

            let parsed = Message::from_str(line.trim()).unwrap();
            trace!("{:?}", parsed);

            if let &Body::Reply(ref rpl) = parsed.body() {
                match rpl.code() {
                    ReplyCode::RPL_MOTD => {
                        let params = rpl.parameters().collect::<Vec<&str>>();
                        println!("-*- {}", params.connect(" "));
                    },
                    ReplyCode::RPL_ENDOFMOTD => {
                        let params = rpl.parameters().collect::<Vec<&str>>();
                        println!("-*- {}", params.connect(" "));
                        break;
                    },
                    _ => {}
                }
            }
        }
    }

    {
        // Join channel
        let joincmd = JoinCommand::new("#rust", None);
        let joinmsg = Message::new(None, Body::command(joincmd));

        info!("Join channel: Sending {:?}", joinmsg);
        write!(stream, "{}\r\n", joinmsg).unwrap();
    }

    let mut writeable_stream = stream.try_clone().unwrap();
    let cloned_username = username.clone();
    thread::spawn(move|| {
        let bufstdin = BufReader::new(io::stdin());
        for line in bufstdin.lines() {
            let line = line.unwrap();
            let msg = line.trim();

            if !msg.is_empty() {
                if msg.starts_with("/") {
                    let mut sp = msg.splitn(2, ' ');
                    let cmd = sp.next().unwrap();
                    let cmd = (&cmd[1..]).chars().flat_map(|x| x.to_uppercase()).collect::<String>();

                    match &cmd[..] {
                        "QUIT" => {
                            let quitcmd = QuitCommand::new(sp.next());
                            let msg = Message::new(None, Body::command(quitcmd));
                            trace!("Quit: Sending {:?}", msg);
                            write!(writeable_stream, "{}\r\n", msg).unwrap();
                        },
                        _ => {
                            let privmsg = PrivmsgCommand::new("#rust", msg);
                            println!("-*- < {} > {}", cloned_username, msg);
                            let msg = Message::new(None, Body::command(privmsg));
                            trace!("Privmsg: Sending {:?}", msg);
                            write!(writeable_stream, "{}\r\n", msg).unwrap();
                        }
                    }
                } else {
                    let privmsg = PrivmsgCommand::new("#rust", msg);
                    println!("-*- < {} > {}", cloned_username, msg);
                    let msg = Message::new(None, Body::command(privmsg));
                    trace!("Privmsg: Sending {:?}", msg);
                    write!(writeable_stream, "{}\r\n", msg).unwrap();
                }
            }
        }
    });

    // Prints conversation
    for line in lines {
        let line = line.unwrap();

        let parsed = Message::from_str(line.trim()).unwrap();
        trace!("{:?}", parsed);

        if let &Body::Command(ref cmd) = parsed.body() {
            match cmd {
                &Command::Privmsg(ref privmsg) => {
                    let prefix = parsed.prefix().unwrap();

                    let name = match prefix.find('!') {
                        None => prefix,
                        Some(idx) => &prefix[..idx],
                    };

                    println!("-*- < {} > {}", name, privmsg.message());
                },
                &Command::Ping(ref ping) => {
                    debug!("Received ping {:?}", ping);
                    let pong = PongCommand::new(ping.servers().get());
                    debug!("Sending pong {:?}", pong);
                    write!(stream, "{}\r\n", Message::new(None, Body::command(pong))).unwrap();
                },
                _ => {}
            }
        }
    }
}
