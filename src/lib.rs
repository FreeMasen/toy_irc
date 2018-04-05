extern crate irc;

use irc::client::prelude::*;

///User entered config items
pub struct Server {
    ///a friendly identifier
    name: String,
    ///The host to connect to
    url: String,
    ///The port to connect to
    port: String,
    ///The user info
    user: User,
    ///The server password
    password: String,
    ///The channels to connect to
    channels: Vec<String>,
}

impl Server {
    // fn new() -> Server {
    //     Server {
    //         name: String::new(),
    //         url: String::new(),
    //         port: String::new(),
    //         user: String::new(),
    //         password: String::new(),
    //         channels: vec!(),
    //     }
    // }

    fn into_config(&self) -> Config {
        let nick_password = if self.user.password.len() > 0 {
            Some(self.user.password.clone())
        } else {
            None
        };
        let alt_nicks = if self.user.alt_nicks.len() > 0 {
            Some(self.user.alt_nicks.clone())
        } else {
            None
        };
        let realname = if self.user.name.len() > 0 {
            Some(self.user.name.clone())
        } else {
            None
        };
        let (server, use_ssl) = if self.url.starts_with("https") {
            (Some(self.url.replace("https://", "")), Some(true))
        } else {
            (Some(self.url.replace("http://", "")), Some(false))
        };
        let password = if self.password.len() > 0 {
            Some(self.password.clone())
        } else {
            None
        };
        let port = if self.port.len() > 0 {
            match self.port.parse() {
                Ok(p) => Some(p),
                _ => None
            }
        } else {
            None
        };
        let channels = if self.channels.len() > 0 {
            Some(self.channels.clone())
        } else {
            None
        };
        Config {
            owners: None,
            nickname: Some(self.user.nickname.clone()),
            nick_password,
            alt_nicks,
            username: Some(self.user.username.clone()),
            realname,
            server,
            port,
            password,
            use_ssl,
            cert_path: None,
            encoding: None,
            channels,
            umodes: None,
            user_info: None,
            version: None,
            source: None,
            ping_time: None,
            ping_timeout: None,
            burst_window_length: None,
            max_messages_in_burst: None,
            should_ghost: None,
            ghost_sequence: None,
            use_mock_connection: None,
            mock_initial_value: None,
            channel_keys: None,
            options: None,
            path: None,
        }
    }
}

///The user specific parts of the config
pub struct User {
    ///realname
    name: String,
    username: String,
    nickname: String,
    ///nick_password
    password: String,
    alt_nicks: Vec<String>,
}

///A message the user has entered
pub struct Message {
    server_id: u32,
    channel_id: u32,
    text: String,
}

///Events sent from the UI
pub enum IncomingEvent {
    ///connect to a server
    Connect(Config),
    ///UI request for messages (server_id, channel_id)
    GetMessages(u32, u32),
    ///The users is
    Typing(u32, u32),
    SendMessage(u32, String),
    GetSesssion,
}

pub struct Session {
    servers: Vec<Server>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            servers: Vec::new(),
        }
    }

    pub fn add_server(&mut self, server: Server) {
        self.servers.push(server);
    }
}

#[cfg(test)]
mod tests {
    
}
