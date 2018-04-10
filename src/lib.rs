extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate irc;



pub mod server;
pub mod channel;
pub mod event;

pub mod prelude {
    pub use server::Server;
    pub use event::Event;
}