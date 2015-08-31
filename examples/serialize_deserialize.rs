extern crate irc;

use irc::protocol::{Message, Body, IrcMessage};
use irc::protocol::command::PrivmsgCommand;

fn main() {

    let data = ":abc!abc@example.com PRIVMSG #rust :Ok, thanks guys. :)";
    let parsed_msg = Message::from_str(data).unwrap();


    let privmsg = PrivmsgCommand::new("#rust", "Ok, thanks guys. :)");
    let expected_msg = Message::new(Some("abc!abc@example.com"), Body::command(privmsg));

    assert_eq!(expected_msg, parsed_msg);

    println!("{}", expected_msg);
}
