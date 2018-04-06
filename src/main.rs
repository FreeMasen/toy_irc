mod server;
use server::*;
// mod irc_client;

// use irc_client::*;
extern crate irc;
extern crate futures;
use irc::client::prelude::*;


fn main() {
    let config = Config {
        nickname: Some("rubot".to_owned()),
        server: Some("irc.mozilla.org".to_owned()),
        channels: Some(vec!["#rust".to_owned(), "#servo".to_owned()]),
        ..Config::default()
    };
    let mut reactor = IrcReactor::new().expect("Unable to create a reactor");
    let client = reactor.prepare_client_and_connect(&config).expect("Create/Connect failed");
    client.identify().expect("identify failed");
    
    let mut server = Server::new();
    
    reactor.register_client_with_handler(client, handler)
    reactor.run(server_events).expect("Unable to run reactor");
}



