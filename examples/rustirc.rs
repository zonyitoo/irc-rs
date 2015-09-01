#[macro_use]
extern crate log;
extern crate env_logger;
extern crate irc;

use std::net::TcpStream;
use std::io::{Write, BufRead, BufReader};

use irc::protocol::message::{IrcMessage, Message, Body};
use irc::protocol::command::{Command, UserCommand, NickCommand, JoinCommand, PongCommand};
use irc::protocol::reply::ReplyCode;

fn main() {
    env_logger::init().unwrap();

    let server_addr = "irc.mozilla.org:6667";

    info!("Connecting to {:?}", server_addr);
    let mut stream = TcpStream::connect(server_addr).unwrap();
    let bufreader = BufReader::new(stream.try_clone().unwrap());

    let username = "rust_irc_test";
    let nickname = username;

    let usercmd = UserCommand::new(username, 0, username);
    let nickcmd = NickCommand::new(nickname);

    {
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
                    println!("-*- {}", params.join(" "));
                    break;
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
                        println!("-*- {}", params.join(" "));
                    },
                    ReplyCode::RPL_ENDOFMOTD => {
                        let params = rpl.parameters().collect::<Vec<&str>>();
                        println!("-*- {}", params.join(" "));
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
