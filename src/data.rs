use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use channel::ChannelMessage;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_chat(chan: String, msg: ChannelMessage) {
    
}

#[derive(Queryable)]
pub struct Chat {
    pub channel: String,
    pub time_stamp: String,
    pub user_name: String,
    pub content: String,
}

impl Chat {
    pub fn from(channel: String, msg: ChannelMessage) -> Chat {
        Chat {
            channel,
            ..msg
        }
    }
}