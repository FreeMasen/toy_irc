use std::collections::{HashMap};
use irc::client::prelude::*;

#[derive(Debug, Clone, Serialize)]
pub struct Channel {
    name: String,
    topic: String,
    users: HashMap<String, ChannelUser>,
    messages: Vec<ChannelMessage>,
    status: Vec<ChannelStatus>
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum ChannelStatus {
    Ban(String),
    Exception,
    Limit,
    InviteOnly,
    InviteException,
    Key,
    Moderated,
    RegisteredOnly,
    Secret,
    ProtectedTopic,
    NoExternalMessages,
    Founder(String)
}

impl ChannelStatus {
    fn from(mode: ChannelMode, arg: Option<String>) -> ChannelStatus {
        match mode {
            ChannelMode::Exception => ChannelStatus::Exception,
            ChannelMode::Limit => ChannelStatus::Limit,
            ChannelMode::InviteOnly => ChannelStatus::InviteOnly,
            ChannelMode::InviteException => ChannelStatus::InviteException,
            ChannelMode::Key => ChannelStatus::Key,
            ChannelMode::Moderated => ChannelStatus::Moderated,
            ChannelMode::RegisteredOnly => ChannelStatus::RegisteredOnly,
            ChannelMode::Secret => ChannelStatus::Secret,
            ChannelMode::ProtectedTopic => ChannelStatus::ProtectedTopic,
            ChannelMode::NoExternalMessages => ChannelStatus::NoExternalMessages,
            ChannelMode::Ban => ChannelStatus::Ban(arg.unwrap_or(String::From("Unknown users"))),
            _ => ChannelStatus::
        }
    }
}


#[derive(Debug, Clone, Serialize)]
pub struct ChannelMessage {
    pub time_stamp: String,
    pub user_name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChannelUser {
    name: String,
    status: Vec<UserStatus>,
}
#[derive(Debug, Clone, Serialize)]
pub enum UserStatus {
    Away,
    Invisible,
    Wallops,
    Restricted,
    Oper(String),
    LocalOper,
    ServerNotices,
    MaskedHosts,
    Unknown(char),
}

impl Channel {
    pub fn new() -> Self {
        Channel {
            name: String::new(),
            topic: String::new(),
            users: HashMap::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_users(&mut self, text: &str) -> u32 {
        let mut count = 0;
        for un in text.split(" ") {
            match self.users.insert(String::from(un), ChannelUser::with_name(&un)) {
                Some(_) => (),
                None => count += 1,
            }
        }
        count
    }

    pub fn add_message(&mut self, msg: ChannelMessage) {
        self.messages.push(msg);
    }

    pub fn set_topic(&mut self, topic: &str) {
        self.topic = String::from(topic);
    }

    pub fn users(&self) -> Vec<String> {
        self.users.iter().map(|u| u.1.name.to_string()).collect()
    }

    pub fn remove_user(&mut self, username: &str) -> bool {
        match self.users.remove(username) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn chan_status(&mut self, mode: Mode<ChannelMode>) {
        match mode {
            Plus(mode, arg) => self.add_status(mode, arg),
            Minus(mode, arg) => self.remove_status(mode, arg),
        }
    }
    fn add_status(&mut self, mode: ChannelMode, arg: Option<String>) {
        match mode {
            ChannelMode::Founder => (),
            ChannelMode::Admin => (),
            ChannelMode::Oper => (),
            ChannelMode::Halfop => (),
            ChannelMode::Voice => (),
            _ => {
                self.status.push(ChannelStatus::from(mode));
                self.status.dedup();
            }
        }
    }

    fn remove_status(&mut self, mode: ChannelMode, arg: Option<String>) {
        match mode {
            ChannelMode::Ban => {match arg {
                Some(name) => self.status.push()
            }},
            ChannelMode::Founder => (),
            ChannelMode::Admin => (),
            ChannelMode::Oper => (),
            ChannelMode::Halfop => (),
            ChannelMode::Voice => (),
            _ => {
                self.status = self.staus.clone().into_iter().filter(|s| s != ChannelStatus::from(mode))
            }
        }
    }

    fn add_user_status(&mut self, mode: UserMode, arg: Option<String>) {

    }

    fn remove_user_status(&mut self, mode: UserMode, arg: Option<String>) {

    }
}

impl ChannelUser {
    pub fn with_name(name: &str) -> ChannelUser {
        ChannelUser {
            name: String::from(name),
            chan_modes: vec![],
            modes: vec![],
        }
    }

    pub fn add_status(&mut self, chan_mode: ChannelMode) {
        self.chan_modes.push(level);
        self.chan_modes.dedup();
    }

    pub fn remove_status(&mut self, chan_mode: ChannelMode) {
        self.chan_modes = self.chan_modes.clone().into_iter().filter(|c| c != &chan_mode).collect();
    }
}