extern crate irc;
extern crate futures;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

extern crate irc_client;

use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use irc_client::prelude::*;
use irc::client::prelude::*;
use serde_json::to_string;

fn main() {
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0));
    write_entry(format!("{{\"type\": \"started\", \"args\": [{}]}}", start_time.as_secs()));
    let config = Config {
        nickname: Some("fruitbot".to_owned()),
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
        Event::NewUsers(ch, users) => write_entry(format!("{{\"type\": \"new-users\", \"args\": [\"{}\", {}]}}", ch, users.len())),
        // Event::NewMessage(ch, message) => print!(",\n\"{} msg: {:?}\"", ch, to_string(&message)),
        _ => {
            match to_string(&ev) {
                Ok(ev_str) => write_entry(format!("{}", ev_str)),
                _ => write_entry(String::from("{\"type\": \"error\", \"args\": [\"Unable to convert ev to json\"]}"))
            }
        },
    }
}

fn write_entry(line: String) {
    let path = PathBuf::from("out.log.json");
    if !path.exists() {
        File::create(&path).expect("Unable to create log file");
    }
    let md = path.metadata().expect("Unable to get file metadata");
    
    let prefix = if md.len() > 0 {
                ",\n"
            } else {
                "[\n"
            };

    let mut f = OpenOptions::new().append(true).open(path).expect("Unable to open file");
    let _size = f.write_all(format!("{}{}", prefix, line).as_bytes()).expect("Unable to write to file");
}