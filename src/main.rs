extern crate irc;
extern crate futures;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

extern crate irc_client;
use irc_client::prelude::*;

use irc::client::prelude::*;
use serde_json::to_string;

fn main() {
    print!("[\"started\"");
    let config = Config {
        nickname: Some("rubot".to_owned()),
        server: Some("irc.mozilla.org".to_owned()),
        channels: Some(vec!["#rust".to_owned(), "#rust_embedded".to_owned()]),
        ..Config::default()
    };
    let client = IrcClient::from_config(config).expect("Unable to create client");
    client.identify().expect("Unable to identify client");
    let mut server = Server::with(Box::new(listener));
    client.for_each_incoming(|msg| {
        server.handle_message(msg);
    }).expect("Unable to register incoming handler");
}


fn listener(ev: Event) {
    match ev {
        // Event::Welcome(text) => print!(",\n\"Welcome: {}\"", text),
        // Event::MOTD(_text) => print!(",\n\"MOTD\""),
        Event::NewUsers(ch, users) => print!(",\n{{\"type\": \"new-users\", \"args\": [\"{}\", {}]}}", ch, users.len()),
        // Event::NewMessage(ch, message) => print!(",\n\"{} msg: {:?}\"", ch, to_string(&message)),
        _ => {
            match to_string(&ev) {
                Ok(ev_str) => print!(",\n{}", ev_str),
                _ => print!(",\n\"Unable to convert ev to json\""),
            }
        },
    }
}