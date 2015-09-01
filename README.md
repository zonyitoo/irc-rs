# IRC-rs

[![Build Status](https://img.shields.io/travis/zonyitoo/irc-rs.svg)](https://travis-ci.org/zonyitoo/irc-rs)
[![License](https://img.shields.io/github/license/zonyitoo/irc-rs.svg)](https://github.com/zonyitoo/irc-rs)

IRC library for Rust

## Usage

### Serialization and Deserialization

```rust
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
```

## TODOs

- [ ] Basically support [RFC2812](https://tools.ietf.org/html/rfc2812)
- [ ] API with validation
- [ ] Supports for clients and servers
- [ ] Sample clients and server implementation
