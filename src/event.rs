use channel::ChannelMessage;
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type", content = "args", rename_all = "kebab-case")]
pub enum Event {
    Welcome(String),
    Motd(String),
    NewUsers(String, Vec<String>),
    NewMessage(String, ChannelMessage),
    Misc(Option<String>, String, Vec<String>, Option<String>)
}