use std::collections::{HashMap};
use std::fmt::{Debug, Result, Formatter};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use irc::client::prelude::*;
use serde_json::to_string;

use event::Event;

use channel::{ChannelMessage, Channel};
#[derive(Serialize)]
pub struct Server {
    welcome_msg: String,
    connection_status: ConnectionStatus,
    motd: String,
    channels: HashMap<String, Channel>,
    #[serde(skip)]
    listener: Box<Fn(Event)>
}

#[derive(Serialize)]
pub enum ConnectionStatus {
    NotConnected,
    Authenticating,
    Connected,
    Idle
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?} {:?}", self.motd, self.channels)
    }
}

impl Server {
    pub fn new() -> Server {
        Server {
            welcome_msg: String::new(),
            connection_status: ConnectionStatus::NotConnected,
            motd: String::new(),
            channels: HashMap::new(),
            listener: Box::new(|_|{}),
        }
    }

    pub fn with(listener: Box<Fn(Event)>) -> Server {
        Server {
            welcome_msg: String::new(),
            connection_status: ConnectionStatus::NotConnected,
            motd: String::new(),
            channels: HashMap::new(),
            listener
        }
    }

    pub fn get_state(&self) -> String {
        to_string(&self).unwrap_or(String::from("{\"type\":\"error\", \"args\": [\"Unable to convert state\"]}"))
    }

    pub fn add_motd(&mut self, text: String) {
        self.motd = if text.ends_with("\n") {
            self.get_motd() + &text
        } else {
            let with_new_line = text + "\n";
            self.get_motd() + &with_new_line
        };
    }

    fn add_welcome(&mut self, text: &str) {
        self.welcome_msg = String::from(text);
    }

    pub fn get_motd(&self) -> String {
        self.motd.clone()
    }

    pub fn add_users(&mut self, channel: &str, names: &str) {
        let ch = self.channels.entry(String::from(channel)).or_insert(Channel::new());
        ch.add_users(names);
    }

    pub fn add_ch_topic(&mut self, channel: &str, topic: &str) {
        let ch = self.channels.entry(String::from(channel)).or_insert(Channel::new());
        ch.set_topic(topic);
    }

    pub fn remove_user(&mut self, username: &str) {
        self.channels = self.channels.clone().into_iter().map(|mut e| {
            if e.1.remove_user(username) {
                (self.listener)(Event::NewUsers(e.0.clone(), e.1.users()));
            }
            e
        }).collect();
    }

    pub fn change_nick(&mut self, old: &str, new: &str) {
        println!("change_nick {}, {}", &old, &new);
        self.remove_user(old);
        self.channels = self.channels.clone().into_iter().map(|mut e| {
            if e.1.add_users(new) > 0 {
                (self.listener)(Event::NewUsers(e.0.clone(), e.1.users()));
            }
            e
        }).collect();
    }

    fn short_name(logn_name: Option<String>) -> String {
        match logn_name {
            Some(p) => {
                let parts: Vec<&str> = p.split('!').collect();
                String::from(parts[0])
            },
            None => String::new()
        }
    }

    #[allow(unused_variables)]
    pub fn handle_message(&mut self, msg: Message) {
        match msg.tags {
            Some(tags) => {
                for tag in tags {
                    println!("tag: {:?}", tag);
                }
            },
            _ => ()
        }
        match msg.command {
            Command::PASS(pwd) => (self.listener)(Event::Misc(msg.prefix, String::from("PASS"), vec![pwd], None)),
            Command::NICK(name) => {
                let old_name = Self::short_name(msg.prefix);
                self.change_nick(&old_name, &name);
                (self.listener)(Event::Misc(None, String::from("NICK"), vec![old_name, name], None))
            },
            Command::USER(user, mode, realname) => (self.listener)(Event::Misc(msg.prefix, String::from("USER"), vec![user, mode, realname], None)),
            Command::OPER(name, pwd) => (self.listener)(Event::Misc(msg.prefix, String::from("OPER"), vec![name, pwd], None)),
            Command::UserMODE(mode, nics) => (self.listener)(Event::Misc(msg.prefix, String::from("UserMODE"), vec![], None)),
            Command::SERVICE(service, nic, reserved, dist, tp, res_info,) => (self.listener)(Event::Misc(msg.prefix, String::from("SERVICE"), vec![], None)),
            Command::QUIT(comment) => {
                let user_name = Self::short_name(msg.prefix);
                self.remove_user(&user_name);
            },
            Command::SQUIT(server, comment) => (self.listener)(Event::Misc(msg.prefix, String::from("SQUIT"), vec![server, comment], None)),
            Command::JOIN(list, keys, realname) => {
                let user_name = Self::short_name(msg.prefix);
                self.add_users(&list, &user_name);
            },
            Command::PART(list, comment) => {
                let user_name = Self::short_name(msg.prefix);
                self.remove_user(&user_name);
                (self.listener)(Event::Misc(Some(user_name), String::from("PART"), vec![list, comment.unwrap_or(String::new())], None))
            },
            Command::ChannelMODE(channel, modes) => (self.listener)(Event::Misc(msg.prefix, String::from("ChannelMODE"), vec![channel, modes.iter().map(|m|format!("{:?}", m)).collect::<Vec<String>>().join(", ")], None)),
            Command::TOPIC(channel, topic) => (self.listener)(Event::Misc(msg.prefix, String::from("TOPIC"), vec![channel, topic.unwrap_or(String::new())], None)),
            Command::NAMES(list, target) => (self.listener)(Event::Misc(msg.prefix, String::from("NAMES"), vec![list.unwrap_or(String::new()), target.unwrap_or(String::new())], None)),
            Command::LIST(list, target) => (self.listener)(Event::Misc(msg.prefix, String::from("LIST"), vec![list.unwrap_or(String::new()), target.unwrap_or(String::new())], None)),
            Command::INVITE(nickname, channel) => (self.listener)(Event::Misc(msg.prefix, String::from("INVITE"),vec![nickname, channel], None)),
            Command::KICK(list, user_list, comment) => (self.listener)(Event::Misc(msg.prefix, String::from("KICK"), vec![list, user_list, comment.unwrap_or(String::new())], None)),
            Command::PRIVMSG(target, text) => self.new_message(msg.prefix, target, text),
            Command::NOTICE(target, text) => {
                if &target == "AUTH" {
                    self.connection_status = ConnectionStatus::Authenticating;
                } else if target.starts_with("#") || target.starts_with("&") {
                    self.new_message(msg.prefix, target, text)
                } else {
                    (self.listener)(Event::Misc(msg.prefix, String::from("NOTICE"), vec![target, text], None))
                }
            },
            Command::MOTD(target) => (self.listener)(Event::Misc(msg.prefix, String::from("MOTD"), vec![target.unwrap_or(String::new())], None)),
            Command::LUSERS(mask, target) => (self.listener)(Event::Misc(msg.prefix, String::from("LUSERS"), vec![mask.unwrap_or(String::new()), target.unwrap_or(String::new())], None)),
            Command::VERSION(version) => (self.listener)(Event::Misc(msg.prefix, String::from("VERSION"), vec![version.unwrap_or(String::new())], None)),
            Command::STATS(query, target) => (self.listener)(Event::Misc(msg.prefix, String::from("STATS"), vec![query.unwrap_or(String::new()), target.unwrap_or(String::new())], None)),
            Command::LINKS(server, mask) => (self.listener)(Event::Misc(msg.prefix, String::from("LINKS"), vec![server.unwrap_or(String::new()), mask.unwrap_or(String::new())], None)),
            Command::TIME(time) => (self.listener)(Event::Misc(msg.prefix, String::from("TIME"), vec![time.unwrap_or(String::new())], None)),
            Command::CONNECT(server, port, remote) => (self.listener)(Event::Misc(msg.prefix, String::from("CONNECT"), vec![server, port, remote.unwrap_or(String::new())], None)),
            Command::TRACE(target) => (self.listener)(Event::Misc(msg.prefix, String::from("TRACE"), vec![target.unwrap_or(String::new())], None)),
            Command::ADMIN(target) => (self.listener)(Event::Misc(msg.prefix, String::from("ADMIN"), vec![target.unwrap_or(String::new())], None)),
            Command::INFO(target) => (self.listener)(Event::Misc(msg.prefix, String::from("INFO"), vec![target.unwrap_or(String::new())], None)),
            Command::SERVLIST(mask, tp) => (self.listener)(Event::Misc(msg.prefix, String::from("SERVLIST"), vec![mask.unwrap_or(String::new()), tp.unwrap_or(String::new())], None)),
            Command::SQUERY(name, text) => (self.listener)(Event::Misc(msg.prefix, String::from("SQUERY"), vec![name, text], None)),
            Command::WHO(mask, operator) => (self.listener)(Event::Misc(msg.prefix, String::from("WHO"), vec![mask.unwrap_or(String::new()), format!("{:?}", operator)], None)),
            Command::WHOIS(target, list) => (self.listener)(Event::Misc(msg.prefix, String::from("WHOIS"), vec![target.unwrap_or(String::new()), list], None)),
            Command::WHOWAS(list, count, target) => (self.listener)(Event::Misc(msg.prefix, String::from("WHOWAS"), vec![list, count.unwrap_or(String::new()), target.unwrap_or(String::new())], None)),
            Command::KILL(name, comment) => (self.listener)(Event::Misc(msg.prefix, String::from("KILL"), vec![name, comment], None)),
            Command::PING(me, you) => (self.listener)(Event::Misc(msg.prefix, String::from("PING"), vec![], None)),
            Command::PONG(me, you) => (self.listener)(Event::Misc(msg.prefix, String::from("PONG"), vec![me, you.unwrap_or(String::new())], None)),
            Command::ERROR(message) => (self.listener)(Event::Misc(msg.prefix, String::from("ERROR"), vec![message], None)),
            Command::AWAY(message) => (self.listener)(Event::Misc(msg.prefix, String::from("AWAY"), vec![message.unwrap_or(String::new())], None)),
            Command::REHASH => (self.listener)(Event::Misc(msg.prefix, String::from("REHASH"), vec![], None)),
            Command::DIE => (self.listener)(Event::Misc(msg.prefix, String::from("DIE"), vec![], None)),
            Command::RESTART => (self.listener)(Event::Misc(msg.prefix, String::from("RESTART"), vec![], None)),
            Command::SUMMON(user, target, channel) => (self.listener)(Event::Misc(msg.prefix, String::from("SUMMON"), vec![user, target.unwrap_or(String::new()), channel.unwrap_or(String::new())], None)),
            Command::USERS(list) => (self.listener)(Event::Misc(msg.prefix, String::from("USERS"), vec![list.unwrap_or(String::new())], None)),
            Command::WALLOPS(text) => (self.listener)(Event::Misc(msg.prefix, String::from("WALLOPS"), vec![ text], None)),
            Command::USERHOST(list) => (self.listener)(Event::Misc(msg.prefix, String::from("USERHOST"), vec![ list.join(", ")], None)),
            Command::ISON(list) => (self.listener)(Event::Misc(msg.prefix, String::from("ISON"), vec![ list.join(" ")], None)),
            Command::SAJOIN(name, channel) => (self.listener)(Event::Misc(msg.prefix, String::from("SAJOIN"), vec![name, channel], None)),
            Command::SAMODE(target, modes, params) => (self.listener)(Event::Misc(msg.prefix, String::from("SAMODE"), vec![target, modes, params.unwrap_or(String::new())], None)),
            Command::SANICK(old, new) => (self.listener)(Event::Misc(msg.prefix, String::from("SANICK"), vec![old, new], None)),
            Command::SAPART(name, comment) => (self.listener)(Event::Misc(msg.prefix, String::from("SAPART"), vec![ name, comment], None)),
            Command::SAQUIT(name, comment) => (self.listener)(Event::Misc(msg.prefix, String::from("SAQUIT"), vec![name, comment], None)),
            Command::NICKSERV(message) => (self.listener)(Event::Misc(msg.prefix, String::from("NICKSERV"), vec![ message], None)),
            Command::CHANSERV(message) => (self.listener)(Event::Misc(msg.prefix, String::from("CHANSERV"), vec![ message], None)),
            Command::OPERSERV(message) => (self.listener)(Event::Misc(msg.prefix, String::from("OPERSERV"), vec![ message], None)),
            Command::BOTSERV(message) => (self.listener)(Event::Misc(msg.prefix, String::from("BOTSERV"), vec![ message], None)),
            Command::HOSTSERV(message) => (self.listener)(Event::Misc(msg.prefix, String::from("HOSTSERV"), vec![ message], None)),
            Command::MEMOSERV(message) => (self.listener)(Event::Misc(msg.prefix, String::from("MEMOSERV"), vec![ message], None)),
            Command::CAP(cmd, sub_cmd, arg, param) => (self.listener)(Event::Misc(msg.prefix, String::from("CAP"), vec![cmd.unwrap_or(String::new()), format!("{:?}", sub_cmd), arg.unwrap_or(String::new()), param.unwrap_or(String::new())], None)),
            Command::AUTHENTICATE(name) => (self.listener)(Event::Misc(msg.prefix, String::from("AUTHENTICATE"), vec![ name], None)),
            Command::ACCOUNT(name) => (self.listener)(Event::Misc(msg.prefix, String::from("ACCOUNT"), vec![ name], None)),
            Command::METADATA(target, sub_cmd, params, param) => (self.listener)(Event::Misc(msg.prefix, String::from("METADATA"), vec![target, format!("{:?}", sub_cmd), format!("{:?}", params), param.unwrap_or(String::new())], None)),
            Command::MONITOR(command, list) => (self.listener)(Event::Misc(msg.prefix, String::from("MONITOR"), vec![command, list.unwrap_or(String::new())], None)),
            Command::BATCH(operator, sub_cmd, params) => {
                let params = if let Some(params) = params {
                    params.join(", ")
                } else {
                    String::new()
                };
                (self.listener)(Event::Misc(msg.prefix, String::from("BATCH"), vec![operator, format!("{:?}", sub_cmd), params], None));
            },
            Command::CHGHOST(user, host) => (self.listener)(Event::Misc(msg.prefix, String::from("CHGHOST"), vec![user, host], None)),
            Command::Response(res, args, suffix) => self.response(res, args, suffix) ,
            Command::Raw(command, params, param) => (self.listener)(Event::Misc(msg.prefix, String::from("Raw"), vec![command, params.join(", "), param.unwrap_or(String::new())], None)),
            
        }
    }

    fn new_message(&mut self, prefix: Option<String>, channel: String, text: String) {
        let user_name = match prefix {
            Some(p) => {
                let parts: Vec<&str> = p.split('!').collect();
                String::from(parts[0])
            },
            None => String::from("Unknown")
        };
        match self.channels.get_mut(&channel) {
            Some(ch) => {
                let time_stamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::new(0, 0)).as_secs();
                let new_message = ChannelMessage {
                    time_stamp: format!("{}", time_stamp),
                    user_name: user_name,
                    content: text,
                };
                ch.add_message(new_message.clone());
                (self.listener)(Event::NewMessage(channel, new_message))
            },
            _ => println!("Unable to get channel {}", &channel)
        }

    }

    fn response(&mut self, res: Response, args: Vec<String>, suffix: Option<String>) {
        match res {
            Response::RPL_WELCOME => {
                let msg = suffix.unwrap_or(String::new());
                self.add_welcome(&msg);
                self.connection_status = ConnectionStatus::Connected;
                (self.listener)(Event::Welcome(msg))
            }
            Response::RPL_AWAY => (self.listener)(Event::Misc(None, String::from("RPL_AWAY"), args, suffix)),
            Response::RPL_UNAWAY => (self.listener)(Event::Misc(None, String::from("RPL_UNAWAY"), args, suffix)),
            Response::RPL_NOWAWAY => (self.listener)(Event::Misc(None, String::from("RPL_UNAWAY"), args, suffix)),
            Response::RPL_WHOISUSER => (self.listener)(Event::Misc(None, String::from("RPL_WHOISUSER"), args, suffix)),
            Response::RPL_WHOISSERVER => (self.listener)(Event::Misc(None, String::from("RPL_WHOISSERVER"), args, suffix)),
            Response::RPL_WHOISOPERATOR => (self.listener)(Event::Misc(None, String::from("RPL_WHOISOPERATOR"), args, suffix)),
            Response::RPL_WHOISIDLE => (self.listener)(Event::Misc(None, String::from("RPL_WHOISIDLE"), args, suffix)),
            Response::RPL_ENDOFWHOIS => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFWHOIS"), args, suffix)),
            Response::RPL_WHOISCHANNELS => (self.listener)(Event::Misc(None, String::from("RPL_WHOISCHANNELS"), args, suffix)),
            Response::RPL_WHOWASUSER => (self.listener)(Event::Misc(None, String::from("RPL_WHOWASUSER"), args, suffix)),
            Response::RPL_ENDOFWHOWAS => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFWHOWAS"), args, suffix)),
            Response::RPL_LISTSTART => (self.listener)(Event::Misc(None, String::from("RPL_LISTSTART"), args, suffix)),
            Response::RPL_LIST => (self.listener)(Event::Misc(None, String::from("RPL_LIST"), args, suffix)),
            Response::RPL_LISTEND => (self.listener)(Event::Misc(None, String::from("RPL_LISTEND"), args, suffix)),
            Response::RPL_UNIQOPIS => (self.listener)(Event::Misc(None, String::from("RPL_UNIQOPIS"), args, suffix)),
            Response::RPL_CHANNELMODEIS => (self.listener)(Event::Misc(None, String::from("RPL_CHANNELMODEIS"), args, suffix)),
            Response::RPL_TOPIC => {
                let channel = match args.iter().last() {
                    Some(ch) => ch,
                    _ => "",
                };
                self.add_ch_topic(&channel, &suffix.unwrap_or(String::new()));
            },
            Response::RPL_TOPICWHOTIME => (),
            Response::RPL_INVITING => (self.listener)(Event::Misc(None, String::from("RPL_INVITING"), args, suffix)),
            Response::RPL_SUMMONING => (self.listener)(Event::Misc(None, String::from("RPL_SUMMONING"), args, suffix)),
            Response::RPL_INVITELIST => (self.listener)(Event::Misc(None, String::from("RPL_INVITELIST"), args, suffix)),
            Response::RPL_ENDOFINVITELIST => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFINVITELIST"), args, suffix)),
            Response::RPL_EXCEPTLIST => (self.listener)(Event::Misc(None, String::from("RPL_EXCEPTLIST"), args, suffix)),
            Response::RPL_ENDOFEXCEPTLIST => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFEXCEPTLIST"), args, suffix)),
            Response::RPL_VERSION => (self.listener)(Event::Misc(None, String::from("RPL_VERSION"), args, suffix)),
            Response::RPL_WHOREPLY => (self.listener)(Event::Misc(None, String::from("RPL_WHOREPLY"), args, suffix)),
            Response::RPL_ENDOFWHO => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFWHO"), args, suffix)),
            Response::RPL_NAMREPLY => {
                let channel = args.into_iter().last().expect("can't get last arg");
                let names = suffix.expect("names suffix is None");
                self.add_users(&channel, &names);
            },
            Response::RPL_ENDOFNAMES => {
                match args.iter().last() {
                    Some(name) => {
                        match self.channels.get(name) {
                            Some(ch) => (self.listener)(Event::NewUsers(name.to_string(), ch.users())),
                            None => ()
                        }
                    },
                    None => ()
                }
            },
            Response::RPL_LINKS => (self.listener)(Event::Misc(None, String::from("RPL_LINKS"), args, suffix)),
            Response::RPL_ENDOFLINKS => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFLINKS"), args, suffix)),
            Response::RPL_BANLIST => (self.listener)(Event::Misc(None, String::from("RPL_BANLIST"), args, suffix)),
            Response::RPL_ENDOFBANLIST => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFBANLIST"), args, suffix)),
            Response::RPL_INFO => (self.listener)(Event::Misc(None, String::from("RPL_INFO"), args, suffix)),
            Response::RPL_ENDOFINFO => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFINFO"), args, suffix)),
            Response::RPL_MOTD => {
                match suffix {
                    Some(text) => self.add_motd(text),
                    _ => ()
                }
            },
            Response::RPL_ENDOFMOTD => (self.listener)(Event::Motd(self.get_motd())),
            Response::RPL_YOUREOPER => (self.listener)(Event::Misc(None, String::from("RPL_YOUREOPER"), args, suffix)),
            Response::RPL_REHASHING => (self.listener)(Event::Misc(None, String::from("RPL_REHASHING"), args, suffix)),
            Response::RPL_YOURESERVICE => (self.listener)(Event::Misc(None, String::from("RPL_YOURESERVICE"), args, suffix)),
            Response::RPL_TIME => (self.listener)(Event::Misc(None, String::from("RPL_TIME"), args, suffix)),
            Response::RPL_USERSSTART => (self.listener)(Event::Misc(None, String::from("RPL_USERSSTART"), args, suffix)),
            Response::RPL_USERS => (self.listener)(Event::Misc(None, String::from("RPL_USERS"), args, suffix)),
            Response::RPL_ENDOFUSERS => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFUSERS"), args, suffix)),
            Response::RPL_NOUSERS => (self.listener)(Event::Misc(None, String::from("RPL_NOUSERS"), args, suffix)),
            Response::RPL_HOSTHIDDEN => (self.listener)(Event::Misc(None, String::from("RPL_HOSTHIDDEN"), args, suffix)),
            Response::RPL_TRACELINK => (self.listener)(Event::Misc(None, String::from("RPL_TRACELINK"), args, suffix)),
            Response::RPL_TRACECONNECTING => (self.listener)(Event::Misc(None, String::from("RPL_TRACECONNECTING"), args, suffix)),
            Response::RPL_TRACEHANDSHAKE => (self.listener)(Event::Misc(None, String::from("RPL_TRACEHANDSHAKE"), args, suffix)),
            Response::RPL_TRACEUKNOWN => (self.listener)(Event::Misc(None, String::from("RPL_TRACEUKNOWN"), args, suffix)),
            Response::RPL_TRACEOPERATOR => (self.listener)(Event::Misc(None, String::from("RPL_TRACEOPERATOR"), args, suffix)),
            Response::RPL_TRACEUSER => (self.listener)(Event::Misc(None, String::from("RPL_TRACEUSER"), args, suffix)),
            Response::RPL_TRACESERVER => (self.listener)(Event::Misc(None, String::from("RPL_TRACESERVER"), args, suffix)),
            Response::RPL_TRACESERVICE => (self.listener)(Event::Misc(None, String::from("RPL_TRACESERVICE"), args, suffix)),
            Response::RPL_TRACENEWTYPE => (self.listener)(Event::Misc(None, String::from("RPL_TRACENEWTYPE"), args, suffix)),
            Response::RPL_TRACECLASS => (self.listener)(Event::Misc(None, String::from("RPL_TRACECLASS"), args, suffix)),
            Response::RPL_TRACERECONNECT => (self.listener)(Event::Misc(None, String::from("RPL_TRACERECONNECT"), args, suffix)),
            Response::RPL_TRACELOG => (self.listener)(Event::Misc(None, String::from("RPL_TRACELOG"), args, suffix)),
            Response::RPL_TRACEEND => (self.listener)(Event::Misc(None, String::from("RPL_TRACEEND"), args, suffix)),
            Response::RPL_STATSLINKINFO => (self.listener)(Event::Misc(None, String::from("RPL_STATSLINKINFO"), args, suffix)),
            Response::RPL_STATSCOMMANDS => (self.listener)(Event::Misc(None, String::from("RPL_STATSCOMMANDS"), args, suffix)),
            Response::RPL_ENDOFSTATS => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFSTATS"), args, suffix)),
            Response::RPL_STATSUPTIME => (self.listener)(Event::Misc(None, String::from("RPL_STATSUPTIME"), args, suffix)),
            Response::RPL_STATSOLINE => (self.listener)(Event::Misc(None, String::from("RPL_STATSOLINE"), args, suffix)),
            Response::RPL_UMODEIS => (self.listener)(Event::Misc(None, String::from("RPL_UMODEIS"), args, suffix)),
            Response::RPL_SERVLIST => (self.listener)(Event::Misc(None, String::from("RPL_SERVLIST"), args, suffix)),
            Response::RPL_SERVLISTEND => (self.listener)(Event::Misc(None, String::from("RPL_SERVLISTEND"), args, suffix)),
            Response::RPL_LUSERCLIENT => (self.listener)(Event::Misc(None, String::from("RPL_LUSERCLIENT"), args, suffix)),
            Response::RPL_LUSEROP => (self.listener)(Event::Misc(None, String::from("RPL_LUSEROP"), args, suffix)),
            Response::RPL_LUSERUNKNOWN => (self.listener)(Event::Misc(None, String::from("RPL_LUSERUNKNOWN"), args, suffix)),
            Response::RPL_LUSERCHANNELS => (self.listener)(Event::Misc(None, String::from("RPL_LUSERCHANNELS"), args, suffix)),
            Response::RPL_LUSERME => (self.listener)(Event::Misc(None, String::from("RPL_LUSERME"), args, suffix)),
            Response::RPL_ADMINME => (self.listener)(Event::Misc(None, String::from("RPL_ADMINME"), args, suffix)),
            Response::RPL_ADMINLOC1 => (self.listener)(Event::Misc(None, String::from("name"), args, suffix)),
            Response::RPL_ADMINLOC2 => (self.listener)(Event::Misc(None, String::from("name"), args, suffix)),
            Response::RPL_ADMINEMAIL => (self.listener)(Event::Misc(None, String::from("RPL_ADMINEMAIL"), args, suffix)),
            Response::RPL_TRYAGAIN => (self.listener)(Event::Misc(None, String::from("RPL_TRYAGAIN"), args, suffix)),
            Response::RPL_LOCALUSERS => (self.listener)(Event::Misc(None, String::from("RPL_LOCALUSERS"), args, suffix)),
            Response::RPL_GLOBALUSERS => (self.listener)(Event::Misc(None, String::from("RPL_GLOBALUSERS"), args, suffix)),
            Response::RPL_WHOISCERTFP => (self.listener)(Event::Misc(None, String::from("RPL_WHOISCERTFP"), args, suffix)),
            Response::RPL_MONONLINE => (self.listener)(Event::Misc(None, String::from("RPL_MONONLINE"), args, suffix)),
            Response::RPL_MONOFFLINE => (self.listener)(Event::Misc(None, String::from("RPL_MONOFFLINE"), args, suffix)),
            Response::RPL_MONLIST => (self.listener)(Event::Misc(None, String::from("RPL_MONLIST"), args, suffix)),
            Response::RPL_ENDOFMONLIST => (self.listener)(Event::Misc(None, String::from("RPL_ENDOFMONLIST"), args, suffix)),
            Response::RPL_WHOISKEYVALUE => (self.listener)(Event::Misc(None, String::from("RPL_WHOISKEYVALUE"), args, suffix)),
            Response::RPL_KEYVALUE => (self.listener)(Event::Misc(None, String::from("RPL_KEYVALUE"), args, suffix)),
            Response::RPL_METADATAEND => (self.listener)(Event::Misc(None, String::from("RPL_METADATAEND"), args, suffix)),
            Response::RPL_LOGGEDIN => (self.listener)(Event::Misc(None, String::from("RPL_LOGGEDIN"), args, suffix)),
            Response::RPL_LOGGEDOUT => (self.listener)(Event::Misc(None, String::from("RPL_LOGGEDOUT"), args, suffix)),
            Response::RPL_SASLSUCCESS => (self.listener)(Event::Misc(None, String::from("RPL_SASLSUCCESS"), args, suffix)),
            Response::RPL_SASLMECHS => (self.listener)(Event::Misc(None, String::from("RPL_SASLMECHS"), args, suffix)),
            Response::ERR_UNKNOWNERROR => (self.listener)(Event::Misc(None, String::from("ERR_UNKNOWNERROR"), args, suffix)),
            Response::ERR_NOSUCHNICK => (self.listener)(Event::Misc(None, String::from("ERR_NOSUCHNICK"), args, suffix)),
            Response::ERR_NOSUCHSERVER => (self.listener)(Event::Misc(None, String::from("ERR_NOSUCHSERVER"), args, suffix)),
            Response::ERR_NOSUCHCHANNEL => (self.listener)(Event::Misc(None, String::from("ERR_NOSUCHCHANNEL"), args, suffix)),
            Response::ERR_CANNOTSENDTOCHAN => (self.listener)(Event::Misc(None, String::from("ERR_CANNOTSENDTOCHAN"), args, suffix)),
            Response::ERR_TOOMANYCHANNELS => (self.listener)(Event::Misc(None, String::from("ERR_TOOMANYCHANNELS"), args, suffix)),
            Response::ERR_WASNOSUCHNICK => (self.listener)(Event::Misc(None, String::from("ERR_WASNOSUCHNICK"), args, suffix)),
            Response::ERR_TOOMANYTARGETS => (self.listener)(Event::Misc(None, String::from("ERR_TOOMANYTARGETS"), args, suffix)),
            Response::ERR_NOSUCHSERVICE => (self.listener)(Event::Misc(None, String::from("ERR_NOSUCHSERVICE"), args, suffix)),
            Response::ERR_NOORIGIN => (self.listener)(Event::Misc(None, String::from("ERR_NOORIGIN"), args, suffix)),
            Response::ERR_NORECIPIENT => (self.listener)(Event::Misc(None, String::from("ERR_NORECIPIENT"), args, suffix)),
            Response::ERR_NOTEXTTOSEND => (self.listener)(Event::Misc(None, String::from("ERR_NOTEXTTOSEND"), args, suffix)),
            Response::ERR_NOTOPLEVEL => (self.listener)(Event::Misc(None, String::from("ERR_NOTOPLEVEL"), args, suffix)),
            Response::ERR_WILDTOPLEVEL => (self.listener)(Event::Misc(None, String::from("ERR_WILDTOPLEVEL"), args, suffix)),
            Response::ERR_BADMASK => (self.listener)(Event::Misc(None, String::from("ERR_BADMASK"), args, suffix)),
            Response::ERR_UNKNOWNCOMMAND => (self.listener)(Event::Misc(None, String::from("ERR_UNKNOWNCOMMAND"), args, suffix)),
            Response::ERR_NOMOTD => (self.listener)(Event::Misc(None, String::from("ERR_NOMOTD"), args, suffix)),
            Response::ERR_NOADMININFO => (self.listener)(Event::Misc(None, String::from("ERR_NOADMININFO"), args, suffix)),
            Response::ERR_FILEERROR => (self.listener)(Event::Misc(None, String::from("ERR_FILEERROR"), args, suffix)),
            Response::ERR_NONICKNAMEGIVEN => (self.listener)(Event::Misc(None, String::from("ERR_NONICKNAMEGIVEN"), args, suffix)),
            Response::ERR_ERRONEOUSNICKNAME => (self.listener)(Event::Misc(None, String::from("ERR_ERRONEOUSNICKNAME"), args, suffix)),
            Response::ERR_NICKNAMEINUSE => (self.listener)(Event::Misc(None, String::from("ERR_NICKNAMEINUSE"), args, suffix)),
            Response::ERR_NICKCOLLISION => (self.listener)(Event::Misc(None, String::from("ERR_NICKCOLLISION"), args, suffix)),
            Response::ERR_UNAVAILRESOURCE => (self.listener)(Event::Misc(None, String::from("ERR_UNAVAILRESOURCE"), args, suffix)),
            Response::ERR_USERNOTINCHANNEL => (self.listener)(Event::Misc(None, String::from("ERR_USERNOTINCHANNEL"), args, suffix)),
            Response::ERR_NOTONCHANNEL => (self.listener)(Event::Misc(None, String::from("ERR_NOTONCHANNEL"), args, suffix)),
            Response::ERR_USERONCHANNEL => (self.listener)(Event::Misc(None, String::from("ERR_USERONCHANNEL"), args, suffix)),
            Response::ERR_NOLOGIN => (self.listener)(Event::Misc(None, String::from("ERR_NOLOGIN"), args, suffix)),
            Response::ERR_SUMMONDISABLED => (self.listener)(Event::Misc(None, String::from("ERR_SUMMONDISABLED"), args, suffix)),
            Response::ERR_USERSDISABLED => (self.listener)(Event::Misc(None, String::from("ERR_USERSDISABLED"), args, suffix)),
            Response::ERR_NOTREGISTERED => (self.listener)(Event::Misc(None, String::from("ERR_NOTREGISTERED"), args, suffix)),
            Response::ERR_NEEDMOREPARAMS => (self.listener)(Event::Misc(None, String::from("ERR_NEEDMOREPARAMS"), args, suffix)),
            Response::ERR_ALREADYREGISTRED => (self.listener)(Event::Misc(None, String::from("ERR_ALREADYREGISTRED"), args, suffix)),
            Response::ERR_NOPERMFORHOST => (self.listener)(Event::Misc(None, String::from("ERR_NOPERMFORHOST"), args, suffix)),
            Response::ERR_PASSWDMISMATCH => (self.listener)(Event::Misc(None, String::from("ERR_PASSWDMISMATCH"), args, suffix)),
            Response::ERR_YOUREBANNEDCREEP => (self.listener)(Event::Misc(None, String::from("ERR_YOUREBANNEDCREEP"), args, suffix)),
            Response::ERR_YOUWILLBEBANNED => (self.listener)(Event::Misc(None, String::from("ERR_YOUWILLBEBANNED"), args, suffix)),
            Response::ERR_KEYSET => (self.listener)(Event::Misc(None, String::from("ERR_KEYSET"), args, suffix)),
            Response::ERR_CHANNELISFULL => (self.listener)(Event::Misc(None, String::from("ERR_CHANNELISFULL"), args, suffix)),
            Response::ERR_UNKNOWNMODE => (self.listener)(Event::Misc(None, String::from("ERR_UNKNOWNMODE"), args, suffix)),
            Response::ERR_INVITEONLYCHAN => (self.listener)(Event::Misc(None, String::from("ERR_INVITEONLYCHAN"), args, suffix)),
            Response::ERR_BANNEDFROMCHAN => (self.listener)(Event::Misc(None, String::from("ERR_BANNEDFROMCHAN"), args, suffix)),
            Response::ERR_BADCHANNELKEY => (self.listener)(Event::Misc(None, String::from("ERR_BADCHANNELKEY"), args, suffix)),
            Response::ERR_BADCHANMASK => (self.listener)(Event::Misc(None, String::from("ERR_BADCHANMASK"), args, suffix)),
            Response::ERR_NOCHANMODES => (self.listener)(Event::Misc(None, String::from("ERR_NOCHANMODES"), args, suffix)),
            Response::ERR_BANLISTFULL => (self.listener)(Event::Misc(None, String::from("ERR_BANLISTFULL"), args, suffix)),
            Response::ERR_NOPRIVILEGES => (self.listener)(Event::Misc(None, String::from("ERR_NOPRIVILEGES"), args, suffix)),
            Response::ERR_CHANOPRIVSNEEDED => (self.listener)(Event::Misc(None, String::from("ERR_CHANOPRIVSNEEDED"), args, suffix)),
            Response::ERR_CANTKILLSERVER => (self.listener)(Event::Misc(None, String::from("ERR_CANTKILLSERVER"), args, suffix)),
            Response::ERR_RESTRICTED => (self.listener)(Event::Misc(None, String::from("ERR_RESTRICTED"), args, suffix)),
            Response::ERR_UNIQOPPRIVSNEEDED => (self.listener)(Event::Misc(None, String::from("ERR_UNIQOPPRIVSNEEDED"), args, suffix)),
            Response::ERR_NOOPERHOST => (self.listener)(Event::Misc(None, String::from("ERR_NOOPERHOST"), args, suffix)),
            Response::ERR_UMODEUNKNOWNFLAG => (self.listener)(Event::Misc(None, String::from("ERR_UMODEUNKNOWNFLAG"), args, suffix)),
            Response::ERR_USERSDONTMATCH => (self.listener)(Event::Misc(None, String::from("ERR_USERSDONTMATCH"), args, suffix)),
            Response::ERR_NOPRIVS => (self.listener)(Event::Misc(None, String::from("ERR_NOPRIVS"), args, suffix)),
            Response::ERR_MONLISTFULL => (self.listener)(Event::Misc(None, String::from("ERR_MONLISTFULL"), args, suffix)),
            Response::ERR_METADATALIMIT => (self.listener)(Event::Misc(None, String::from("ERR_METADATALIMIT"), args, suffix)),
            Response::ERR_TARGETINVALID => (self.listener)(Event::Misc(None, String::from("ERR_TARGETINVALID"), args, suffix)),
            Response::ERR_NOMATCHINGKEY => (self.listener)(Event::Misc(None, String::from("ERR_NOMATCHINGKEY"), args, suffix)),
            Response::ERR_KEYINVALID => (self.listener)(Event::Misc(None, String::from("ERR_KEYINVALID"), args, suffix)),
            Response::ERR_KEYNOTSET => (self.listener)(Event::Misc(None, String::from("ERR_KEYNOTSET"), args, suffix)),
            Response::ERR_KEYNOPERMISSION => (self.listener)(Event::Misc(None, String::from("ERR_KEYNOPERMISSION"), args, suffix)),
            Response::ERR_NICKLOCKED => (self.listener)(Event::Misc(None, String::from("ERR_NICKLOCKED"), args, suffix)),
            Response::ERR_SASLFAIL => (self.listener)(Event::Misc(None, String::from("ERR_SASLFAIL"), args, suffix)),
            Response::ERR_SASLTOOLONG => (self.listener)(Event::Misc(None, String::from("ERR_SASLTOOLONG"), args, suffix)),
            Response::ERR_SASLABORT => (self.listener)(Event::Misc(None, String::from("ERR_SASLABORT"), args, suffix)),
            Response::ERR_SASLALREADY => (self.listener)(Event::Misc(None, String::from("ERR_SASLALREADY"), args, suffix)),
            _ => ()
        }
    }
}