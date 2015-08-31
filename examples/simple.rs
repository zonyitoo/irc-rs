
extern crate irc;

use std::net::TcpStream;
use std::thread;
use std::io::{Write, BufRead, BufReader};

use irc::protocol::message::{RawMessage, IrcMessage, Message, MessageCommand};
use irc::protocol::command::{Command, UserCommand, NickCommand, JoinCommand, PongCommand};

fn main() {
    let mut stream = TcpStream::connect("irc.mozilla.org:6667").unwrap();
    let bufreader = BufReader::new(stream.try_clone().unwrap());

    let usercmd = UserCommand::new("rust_irc_test", 0, "rust_irc_test");
    let nickcmd = NickCommand::new("rust_irc_test");

    let usermsg = Message::new(None, MessageCommand::Command(Command::User(usercmd)));
    let nickmsg = Message::new(None, MessageCommand::Command(Command::Nick(nickcmd)));

    println!("Sending {:?}", usermsg);
    write!(stream, "{}\r\n", usermsg).unwrap();

    println!("Sending {:?}", nickmsg);
    write!(stream, "{}\r\n", nickmsg).unwrap();

    let mut cloned_stream = stream.try_clone().unwrap();
    thread::spawn(move|| {
        thread::sleep_ms(2000);

        let joincmd = JoinCommand::new("#rust", None);
        let joinmsg = Message::new(None, MessageCommand::Command(Command::Join(joincmd)));

        println!("Sending {:?}", joinmsg);
        write!(cloned_stream, "{}\r\n", joinmsg).unwrap();
    });

    println!("Receiving");
    for line in bufreader.lines() {
        let line = line.unwrap();

        println!("line:   {:?}", line.trim());
        let msg = RawMessage::from_str(line.trim()).unwrap();
        println!("msg:    {:?}", msg);

        let parsed = Message::from_raw(&msg);
        println!("parsed: {:?}\n", parsed);

        if let Ok(msg) = parsed {
            if let &MessageCommand::Command(ref cmd) = msg.command() {
                if let &Command::Ping(ref msg) = cmd {
                    let pongcmd = PongCommand::new(msg.servers());
                    let pongmsg = Message::new(None, MessageCommand::Command(Command::Pong(pongcmd)));

                    println!("Sending {:?}", pongmsg);
                    write!(stream, "{}\r\n", pongmsg).unwrap();
                }
            }
        }
    }
}