use std::collections::{HashMap};

#[derive(Debug, Clone, Serialize)]
pub struct Channel {
    name: String,
    topic: String,
    users: HashMap<String, ChannelUser>,
    messages: Vec<ChannelMessage>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChannelMessage {
    pub time_stamp: String,
    pub user_name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Hash, Eq, PartialEq)]
pub struct ChannelUser {
    name: String,
    status: UserStatus,
}

#[derive(Debug, Clone, Serialize, Hash, Eq, PartialEq)]
pub enum UserStatus {
    Unknown = 0,
    Offline = 1,
    Away = 2,
    Online = 3 
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

    pub fn add_users(&mut self, text: &str) {
        for un in text.split(" ") {
            self.users.insert(String::from(un), ChannelUser::with_name(&un));
        }
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

    pub fn remove_user(&mut self, username: &str) {
        self.users.remove(username);
    }
}

impl ChannelUser {
    pub fn with_name(name: &str) -> ChannelUser {
        ChannelUser {
            name: String::from(name),
            status: UserStatus::Unknown,
        }
    }
}