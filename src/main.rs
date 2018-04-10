mod server;
use server::*;
// mod irc_client;

// use irc_client::*;
extern crate irc;
extern crate futures;
extern crate tokio_core;
use irc::client::prelude::*;

fn main() {
    let config = Config {
        nickname: Some("rubot".to_owned()),
        server: Some("irc.mozilla.org".to_owned()),
        channels: Some(vec!["#rust".to_owned(), "#servo".to_owned()]),
        ..Config::default()
    };
    let client = IrcClient::from_config(config).expect("Unable to create client");
    client.identify().expect("Unable to identify client");
    let mut server = Server::with_listener(Box::new(listener));
    client.for_each_incoming(|msg| {
        server.handle_message(msg);
    }).expect("Unable to register incoming handler");

}

fn listener(ev: Event) {
    match ev {
        Event::Welcome(msg) => println!("Welcome {}", msg),
        Event::MOTD(text) => {
            println!("MOTD\n-----------\n");
            for line in text.lines() {
                println!("{}", line);
            }
        },
        Event::NewUsers(name, channel) => println!("{} new users in {}", channel.len(), name),
        Event::NewMessage(channel, message) => println!("{} {}: {}", channel, message.user_name, message.content),
        Event::Misc(name, args, suffix) => println!("Misc: {}\n\targs:{}\n\tsuffix{}", name, args.join(","), suffix.unwrap_or(String::new())),
        _ => println!("Unknown Event"),
    }
}

