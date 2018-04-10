use std::collections::{HashSet, HashMap};
use irc::client::prelude::*;
use std::fmt::{Debug, Result, Formatter};
pub enum Event {
    Welcome(String),
    MOTD(String),
    NewUsers(String, Vec<String>),
    NewMessage(String, ChannelMessage),
    Misc(String, Vec<String>, Option<String>)
}


pub struct Server {
    motd: String,
    channels: HashMap<String, Channel>,
    listener: Box<Fn(Event)>
}

impl Debug for Server {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?} {:?}", self.motd, self.channels)
    }
}

#[derive(Debug, Clone)]
pub struct Channel {
    name: String,
    topic: String,
    users: HashSet<String>,
    messages: Vec<ChannelMessage>,
}

#[derive(Debug, Clone)]
pub struct ChannelMessage {
    pub user_name: String,
    pub content: String,
}

impl Server {
    pub fn new() -> Server {
        Server {
            motd: String::new(),
            channels: HashMap::new(),
            listener: Box::new(|_|{}),
        }
    }

    pub fn with_listener(listener: Box<Fn(Event)>) -> Server {
        Server {
            motd: String::new(),
            channels: HashMap::new(),
            listener,
        }
    }

    pub fn add_motd(&mut self, text: String) {
        self.motd = if text.ends_with("\n") {
            self.get_motd() + &text
        } else {
            let with_new_line = text + "\n";
            self.get_motd() + &with_new_line
        };
    }

    pub fn get_motd(&self) -> String {
        self.motd.clone()
    }

    // pub fn add_channel(&mut self, name: &str) {
    //     self.channels.insert(String::from(name), Channel::new());
    // }

    // pub fn add_user(&mut self, channel: &str, name: &str) {
    //     let ch = self.channels.entry(String::from(channel)).or_insert(Channel::new());
    //     ch.users.insert(String::from(name));
    // }

    pub fn add_users(&mut self, channel: &str, names: &str) {
        let ch = self.channels.entry(String::from(channel)).or_insert(Channel::new());
        ch.add_users(names)
    }

    pub fn add_ch_topic(&mut self, channel: &str, topic: &str) {
        let mut ch = self.channels.entry(String::from(channel)).or_insert(Channel::new());
        ch.topic = String::from(topic);
    }

    // pub fn get_channels(&self) -> HashMap<String, Channel> {
    //     self.channels.clone()
    // }

    pub fn handle_message(&mut self, msg: Message) {
        match msg.command {
            Command::PASS(pwd) => (self.listener)(Event::Misc(String::from("PASS"), vec![pwd], None)),
            Command::NICK(name) => (self.listener)(Event::Misc(String::from("NICK"), vec![name], None)),
            Command::USER(user, mode, realname) => (self.listener)(Event::Misc(String::from("USER "), vec![user, mode, realname], None)),
            Command::OPER(name, pwd) => (),//println!("OPER {} {}", name, pwd),
            Command::UserMODE(mode, nics) => {
                let nicks: Vec<String> = nics.into_iter().map(|m| format!("{:?}", m)).collect();
                ();//println!("UserMODE {}{:?}", mode, nicks.join(""));
            },
            Command::SERVICE(service, nic, reserved, dist, tp, res_info,) => (),//println!("SERVICE {}, {}, {}, {}, {}, {}", service, nic, reserved, dist, tp, res_info),
            Command::QUIT(comment) => (),//println!("QUIT {:?}", comment),
            Command::SQUIT(server, comment) => (),//println!("SQUIT {}, {:?}", server, comment),
            Command::JOIN(list, keys, realname) => (),//println!("JOIN {}, {:?}, {:?}", list, keys, realname),
            Command::PART(list, comment) => (),//println!("PART {}, {:?}", list, comment),
            Command::ChannelMODE(channel, modes) => {
                let modes: Vec<String> = modes.into_iter().map(|m| format!("{:?}", m)).collect();
                ();//println!("ChannelMODE {},{}", channel, modes.join(""));
            },
            Command::TOPIC(channel, topic) => (),//println!("TOPIC {}, {:?}", channel, topic),
            Command::NAMES(list, target) => (),//println!("NAMES {:?}, {:?}", list, target),
            Command::LIST(list, target) => (),//println!("LIST {:?}, {:?}", list, target),
            Command::INVITE(nickname, channel) => (),//println!("INVITE {:?}, {:?}",nickname, channel),
            Command::KICK(list, user_list, comment) => (),//println!("KICK {}, {}, {:?}", list, user_list, comment),
            Command::PRIVMSG(target, text) => self.new_message(msg.prefix, target, text),
            Command::NOTICE(target, text) => (),//println!("NOTICE {} {}", target, text),
            Command::MOTD(target) => (),//println!("MOTD {:?}", target),
            Command::LUSERS(mask, target) => (),//println!("LUSERS {:?}, {:?}", mask, target),
            Command::VERSION(version) => (),//println!("VERSION {:?}", version),
            Command::STATS(query, target) => (),//println!("STATS {:?}, {:?}", query, target),
            Command::LINKS(server, mask) => (),//println!("LINKS {:?}, {:?}", server, mask),
            Command::TIME(time) => (),//println!("TIME {:?}", time),
            Command::CONNECT(server, port, remote) => (),//println!("CONNECT {:}:{:}, {:?}", server, port, remote),
            Command::TRACE(target) => (),//println!("TRACE {:?}", target),
            Command::ADMIN(target) => (),//println!("ADMIN {:?}", target),
            Command::INFO(target) => (),//println!("INFO {:?}", target),
            Command::SERVLIST(mask, tp) => (),//println!("SERVLIST {:?}, {:?}", mask, tp),
            Command::SQUERY(name, text) => (),//println!("SQUERY {}, {}", name, text),
            Command::WHO(mask, operator) => (),//println!("WHO {:?}, {:?}", mask, operator),
            Command::WHOIS(target, list) => (),//println!("WHOIS {:?}  {:?}", target, list),
            Command::WHOWAS(list, count, target) => (),//println!("WHOWAS {}, {:?}, {:?}", list, count, target),
            Command::KILL(name, comment) => (),//println!("KILL {}, {}", name, comment),
            Command::PING(me, you) => (),//println!("PING {}, {:?}", me, you),
            Command::PONG(me, you) => (),//println!("PONG {}, {:?}", me, you),
            Command::ERROR(msg) => (),//println!("ERROR {}", msg),
            Command::AWAY(msg) => (),//println!("AWAY {:?}", msg),
            Command::REHASH => (),
            Command::DIE => (),
            Command::RESTART => (),
            Command::SUMMON(user, target, channel) => (),//println!("SUMMON {}, {:?}, {:?}", user, target, channel),
            Command::USERS(list) => (),//println!("USERS {:?}", list),
            Command::WALLOPS(text) => (),//println!("WALLOPS {}", text),
            Command::USERHOST(list) => (),//println!("USERHOST {}", list.join(", ")),
            Command::ISON(list) => (),//println!("ISON {}", list.join(" ")),
            Command::SAJOIN(name, channel) => (),//println!("SAJOIN {}, {}", name, channel),
            Command::SAMODE(target, modes, params) => (),//println!("SAMODE {}, {}, {:?}", target, modes, params),
            Command::SANICK(old, new) => (),//println!("SANICK {}, {}", old, new),
            Command::SAPART(name, comment) => (),//println!("SAPART {} {}", name, comment),
            Command::SAQUIT(name, comment) => (),//println!("SAQUIT {}, {}", name, comment),
            Command::NICKSERV(msg) => (),//println!("NICKSERV {}", msg),
            Command::CHANSERV(msg) => (),//println!("CHANSERV {}", msg),
            Command::OPERSERV(msg) => (),//println!("OPERSERV {}", msg),
            Command::BOTSERV(msg) => (),//println!("BOTSERV {}", msg),
            Command::HOSTSERV(msg) => (),//println!("HOSTSERV {}", msg),
            Command::MEMOSERV(msg) => (),//println!("MEMOSERV {}", msg),
            Command::CAP(cmd, sub_cmd, arg, param) => (),//println!("CAP {:?}, {:?}. {:?}. {:?}", cmd, sub_cmd, arg, param),
            Command::AUTHENTICATE(name) => (),//println!("AUTHENTICATE {}", name),
            Command::ACCOUNT(name) => (),//println!("ACCOUNT {}", name),
            Command::METADATA(target, sub_cmd, params, param) => {
                //println!("METADATA {} {:?}, {}, {:?}", target, sub_cmd, params, param);
            },
            Command::MONITOR(command, list) => (),//println!("MONITOR {}, {:?}", command, list),
            Command::BATCH(operator, sub_cmd, params) => {
                let params = if let Some(params) = params {
                    params.join(", ")
                } else {
                    String::new()
                };
                //println!("BATCH {} {:?}, {:?}", operator, sub_cmd, params);
            },
            Command::CHGHOST(user, host) => (),//println!("CHGHOST {}, {}", user, host),
            Command::Response(res, args, suffix) => self.response(res, args, suffix) ,
            Command::Raw(command, params, param) => (),//println!("Raw {}, {}, {:?}", command, params.join(", "), param),
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
            Some(mut ch) => {
                let new_message = ChannelMessage {
                    user_name,
                    content: text,
                };
                ch.messages.push(new_message.clone());
                (self.listener)(Event::NewMessage(channel, new_message));
            },
            _ => println!("Unable to get channel {}", &channel)
        }

    }

    fn welcome(&self, args: Vec<String>, suffix: Option<String>) {
        (self.listener)(Event::Welcome(suffix.unwrap_or(String::new())));
    }

    fn response(&mut self, res: Response, args: Vec<String>, suffix: Option<String>) {
        match res {
            Response::RPL_WELCOME => self.welcome(args, suffix),
            Response::RPL_YOURHOST => (self.listener)(Event::Misc(String::from("RPL_YOURHOST"), args, suffix)),
            Response::RPL_CREATED => (self.listener)(Event::Misc(String::from("RPL_CREATED"), args, suffix)),
            Response::RPL_MYINFO => (self.listener)(Event::Misc(String::from("RPL_MYINFO"), args, suffix)),
            Response::RPL_ISUPPORT => (self.listener)(Event::Misc(String::from("RPL_ISUPPORT"), args, suffix)),
            Response::RPL_BOUNCE => (self.listener)(Event::Misc(String::from("RPL_BOUNCE"), args, suffix)),
            Response::RPL_NONE => (self.listener)(Event::Misc(String::from("RPL_NONE"), args, suffix)),
            Response::RPL_USERHOST => (self.listener)(Event::Misc(String::from("RPL_USERHOST"), args, suffix)),
            Response::RPL_ISON => (self.listener)(Event::Misc(String::from("RPL_ISON"), args, suffix)),
            Response::RPL_AWAY => (self.listener)(Event::Misc(String::from("RPL_AWAY"), args, suffix)),
            Response::RPL_UNAWAY => (self.listener)(Event::Misc(String::from("RPL_UNAWAY"), args, suffix)),
            Response::RPL_NOWAWAY => (self.listener)(Event::Misc(String::from("RPL_NOWAWAY"), args, suffix)),
            Response::RPL_WHOISUSER => (self.listener)(Event::Misc(String::from("RPL_WHOISUSER"), args, suffix)),
            Response::RPL_WHOISSERVER => (self.listener)(Event::Misc(String::from("RPL_WHOISSERVER"), args, suffix)),
            Response::RPL_WHOISOPERATOR => (self.listener)(Event::Misc(String::from("RPL_WHOISOPERATOR"), args, suffix)),
            Response::RPL_WHOISIDLE => (self.listener)(Event::Misc(String::from("RPL_WHOISIDLE"), args, suffix)),
            Response::RPL_ENDOFWHOIS => (self.listener)(Event::Misc(String::from("RPL_ENDOFWHOIS"), args, suffix)),
            Response::RPL_WHOISCHANNELS => (self.listener)(Event::Misc(String::from("RPL_WHOISCHANNELS"), args, suffix)),
            Response::RPL_WHOWASUSER => (self.listener)(Event::Misc(String::from("RPL_WHOWASUSER"), args, suffix)),
            Response::RPL_ENDOFWHOWAS => (self.listener)(Event::Misc(String::from("RPL_ENDOFWHOWAS"), args, suffix)),
            Response::RPL_LISTSTART => (self.listener)(Event::Misc(String::from("RPL_LISTSTART"), args, suffix)),
            Response::RPL_LIST => (self.listener)(Event::Misc(String::from("RPL_LIST"), args, suffix)),
            Response::RPL_LISTEND => (self.listener)(Event::Misc(String::from("RPL_LISTEND"), args, suffix)),
            Response::RPL_UNIQOPIS => (self.listener)(Event::Misc(String::from("RPL_UNIQOPIS"), args, suffix)),
            Response::RPL_CHANNELMODEIS => (self.listener)(Event::Misc(String::from("RPL_CHANNELMODEIS"), args, suffix)),
            Response::RPL_NOTOPIC => (),
            Response::RPL_TOPIC => {
                let channel = match args.iter().last() {
                    Some(ch) => ch,
                    _ => "",
                };
                self.add_ch_topic(&channel, &suffix.unwrap_or(String::new()));
            },
            Response::RPL_TOPICWHOTIME => (),
            Response::RPL_INVITING => (self.listener)(Event::Misc(String::from("RPL_INVITING"), args, suffix)),
            Response::RPL_SUMMONING => (self.listener)(Event::Misc(String::from("RPL_SUMMONING"), args, suffix)),
            Response::RPL_INVITELIST => (self.listener)(Event::Misc(String::from("RPL_INVITELIST"), args, suffix)),
            Response::RPL_ENDOFINVITELIST => (self.listener)(Event::Misc(String::from("RPL_ENDOFINVITELIST"), args, suffix)),
            Response::RPL_EXCEPTLIST => (self.listener)(Event::Misc(String::from("RPL_EXCEPTLIST"), args, suffix)),
            Response::RPL_ENDOFEXCEPTLIST => (self.listener)(Event::Misc(String::from("RPL_ENDOFEXCEPTLIST"), args, suffix)),
            Response::RPL_VERSION => (self.listener)(Event::Misc(String::from("RPL_VERSION"), args, suffix)),
            Response::RPL_WHOREPLY => (self.listener)(Event::Misc(String::from("RPL_WHOREPLY"), args, suffix)),
            Response::RPL_ENDOFWHO => (self.listener)(Event::Misc(String::from("RPL_ENDOFWHO"), args, suffix)),
            Response::RPL_NAMREPLY => {
                let channel = args.into_iter().last().expect("can't get last arg");
                let names = suffix.expect("names suffix is None");
                self.add_users(&channel, &names);
            },
            Response::RPL_ENDOFNAMES => {
                match args.iter().last() {
                    Some(name) => {
                        match self.channels.get(name) {
                            Some(ch) => (self.listener)(Event::NewUsers(name.to_string(), ch.users.iter().map(|s| s.to_string()).collect())),
                            None => ()
                        }
                    },
                    None => ()
                }
            },
            Response::RPL_LINKS => (self.listener)(Event::Misc(String::from("RPL_LINKS"), args, suffix)),
            Response::RPL_ENDOFLINKS => (self.listener)(Event::Misc(String::from("RPL_ENDOFLINKS"), args, suffix)),
            Response::RPL_BANLIST => (self.listener)(Event::Misc(String::from("RPL_BANLIST"), args, suffix)),
            Response::RPL_ENDOFBANLIST => (self.listener)(Event::Misc(String::from("RPL_ENDOFBANLIST"), args, suffix)),
            Response::RPL_INFO => (self.listener)(Event::Misc(String::from("RPL_INFO"), args, suffix)),
            Response::RPL_ENDOFINFO => (self.listener)(Event::Misc(String::from("RPL_ENDOFINFO"), args, suffix)),
            Response::RPL_MOTDSTART => (),
            Response::RPL_MOTD => {
                match suffix {
                    Some(text) => self.add_motd(text),
                    _ => ()
                }
            },
            Response::RPL_ENDOFMOTD => (self.listener)(Event::MOTD(self.get_motd())),
            Response::RPL_YOUREOPER => (self.listener)(Event::Misc(String::from("RPL_YOUREOPER"), args, suffix)),
            Response::RPL_REHASHING => (self.listener)(Event::Misc(String::from("RPL_REHASHING"), args, suffix)),
            Response::RPL_YOURESERVICE => (self.listener)(Event::Misc(String::from("RPL_YOURESERVICE"), args, suffix)),
            Response::RPL_TIME => (self.listener)(Event::Misc(String::from("RPL_TIME"), args, suffix)),
            Response::RPL_USERSSTART => (self.listener)(Event::Misc(String::from("RPL_USERSSTART"), args, suffix)),
            Response::RPL_USERS => (self.listener)(Event::Misc(String::from("RPL_USERS"), args, suffix)),
            Response::RPL_ENDOFUSERS => (self.listener)(Event::Misc(String::from("RPL_ENDOFUSERS"), args, suffix)),
            Response::RPL_NOUSERS => (self.listener)(Event::Misc(String::from("RPL_NOUSERS"), args, suffix)),
            Response::RPL_HOSTHIDDEN => (self.listener)(Event::Misc(String::from("RPL_HOSTHIDDEN"), args, suffix)),
            Response::RPL_TRACELINK => (self.listener)(Event::Misc(String::from("RPL_TRACELINK"), args, suffix)),
            Response::RPL_TRACECONNECTING => (self.listener)(Event::Misc(String::from("RPL_TRACECONNECTING"), args, suffix)),
            Response::RPL_TRACEHANDSHAKE => (self.listener)(Event::Misc(String::from("RPL_TRACEHANDSHAKE"), args, suffix)),
            Response::RPL_TRACEUKNOWN => (self.listener)(Event::Misc(String::from("RPL_TRACEUKNOWN"), args, suffix)),
            Response::RPL_TRACEOPERATOR => (self.listener)(Event::Misc(String::from("RPL_TRACEOPERATOR"), args, suffix)),
            Response::RPL_TRACEUSER => (self.listener)(Event::Misc(String::from("RPL_TRACEUSER"), args, suffix)),
            Response::RPL_TRACESERVER => (self.listener)(Event::Misc(String::from("RPL_TRACESERVER"), args, suffix)),
            Response::RPL_TRACESERVICE => (self.listener)(Event::Misc(String::from("RPL_TRACESERVICE"), args, suffix)),
            Response::RPL_TRACENEWTYPE => (self.listener)(Event::Misc(String::from("RPL_TRACENEWTYPE"), args, suffix)),
            Response::RPL_TRACECLASS => (self.listener)(Event::Misc(String::from("RPL_TRACECLASS"), args, suffix)),
            Response::RPL_TRACERECONNECT => (self.listener)(Event::Misc(String::from("RPL_TRACERECONNECT"), args, suffix)),
            Response::RPL_TRACELOG => (self.listener)(Event::Misc(String::from("RPL_TRACELOG"), args, suffix)),
            Response::RPL_TRACEEND => (self.listener)(Event::Misc(String::from("RPL_TRACEEND"), args, suffix)),
            Response::RPL_STATSLINKINFO => (self.listener)(Event::Misc(String::from("RPL_STATSLINKINFO"), args, suffix)),
            Response::RPL_STATSCOMMANDS => (self.listener)(Event::Misc(String::from("RPL_STATSCOMMANDS"), args, suffix)),
            Response::RPL_ENDOFSTATS => (self.listener)(Event::Misc(String::from("RPL_ENDOFSTATS"), args, suffix)),
            Response::RPL_STATSUPTIME => (self.listener)(Event::Misc(String::from("RPL_STATSUPTIME"), args, suffix)),
            Response::RPL_STATSOLINE => (self.listener)(Event::Misc(String::from("RPL_STATSOLINE"), args, suffix)),
            Response::RPL_UMODEIS => (self.listener)(Event::Misc(String::from("RPL_UMODEIS"), args, suffix)),
            Response::RPL_SERVLIST => (self.listener)(Event::Misc(String::from("RPL_SERVLIST"), args, suffix)),
            Response::RPL_SERVLISTEND => (self.listener)(Event::Misc(String::from("RPL_SERVLISTEND"), args, suffix)),
            Response::RPL_LUSERCLIENT => (self.listener)(Event::Misc(String::from("RPL_LUSERCLIENT"), args, suffix)),
            Response::RPL_LUSEROP => (self.listener)(Event::Misc(String::from("RPL_LUSEROP"), args, suffix)),
            Response::RPL_LUSERUNKNOWN => (self.listener)(Event::Misc(String::from("RPL_LUSERUNKNOWN"), args, suffix)),
            Response::RPL_LUSERCHANNELS => (self.listener)(Event::Misc(String::from("RPL_LUSERCHANNELS"), args, suffix)),
            Response::RPL_LUSERME => (self.listener)(Event::Misc(String::from("RPL_LUSERME"), args, suffix)),
            Response::RPL_ADMINME => (self.listener)(Event::Misc(String::from("RPL_ADMINME"), args, suffix)),
            Response::RPL_ADMINLOC1 => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::RPL_ADMINLOC2 => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::RPL_ADMINEMAIL => (self.listener)(Event::Misc(String::from("RPL_ADMINEMAIL"), args, suffix)),
            Response::RPL_TRYAGAIN => (self.listener)(Event::Misc(String::from("RPL_TRYAGAIN"), args, suffix)),
            Response::RPL_LOCALUSERS => (self.listener)(Event::Misc(String::from("RPL_LOCALUSERS"), args, suffix)),
            Response::RPL_GLOBALUSERS => (self.listener)(Event::Misc(String::from("RPL_GLOBALUSERS"), args, suffix)),
            Response::RPL_WHOISCERTFP => (self.listener)(Event::Misc(String::from("RPL_WHOISCERTFP"), args, suffix)),
            Response::RPL_MONONLINE => (self.listener)(Event::Misc(String::from("RPL_MONONLINE"), args, suffix)),
            Response::RPL_MONOFFLINE => (self.listener)(Event::Misc(String::from("RPL_MONOFFLINE"), args, suffix)),
            Response::RPL_MONLIST => (self.listener)(Event::Misc(String::from("RPL_MONLIST"), args, suffix)),
            Response::RPL_ENDOFMONLIST => (self.listener)(Event::Misc(String::from("RPL_ENDOFMONLIST"), args, suffix)),
            Response::RPL_WHOISKEYVALUE => (self.listener)(Event::Misc(String::from("RPL_WHOISKEYVALUE"), args, suffix)),
            Response::RPL_KEYVALUE => (self.listener)(Event::Misc(String::from("RPL_KEYVALUE"), args, suffix)),
            Response::RPL_METADATAEND => (self.listener)(Event::Misc(String::from("RPL_METADATAEND"), args, suffix)),
            Response::RPL_LOGGEDIN => (self.listener)(Event::Misc(String::from("RPL_LOGGEDIN"), args, suffix)),
            Response::RPL_LOGGEDOUT => (self.listener)(Event::Misc(String::from("RPL_LOGGEDOUT"), args, suffix)),
            Response::RPL_SASLSUCCESS => (self.listener)(Event::Misc(String::from("RPL_SASLSUCCESS"), args, suffix)),
            Response::RPL_SASLMECHS => (self.listener)(Event::Misc(String::from("RPL_SASLMECHS"), args, suffix)),
            Response::ERR_UNKNOWNERROR => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOSUCHNICK => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOSUCHSERVER => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOSUCHCHANNEL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_CANNOTSENDTOCHAN => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_TOOMANYCHANNELS => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_WASNOSUCHNICK => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_TOOMANYTARGETS => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOSUCHSERVICE => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOORIGIN => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NORECIPIENT => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOTEXTTOSEND => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOTOPLEVEL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_WILDTOPLEVEL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_BADMASK => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_UNKNOWNCOMMAND => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOMOTD => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOADMININFO => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_FILEERROR => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NONICKNAMEGIVEN => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_ERRONEOUSNICKNAME => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NICKNAMEINUSE => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NICKCOLLISION => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_UNAVAILRESOURCE => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_USERNOTINCHANNEL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOTONCHANNEL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_USERONCHANNEL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOLOGIN => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_SUMMONDISABLED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_USERSDISABLED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOTREGISTERED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NEEDMOREPARAMS => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_ALREADYREGISTRED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOPERMFORHOST => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_PASSWDMISMATCH => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_YOUREBANNEDCREEP => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_YOUWILLBEBANNED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_KEYSET => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_CHANNELISFULL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_UNKNOWNMODE => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_INVITEONLYCHAN => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_BANNEDFROMCHAN => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_BADCHANNELKEY => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_BADCHANMASK => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOCHANMODES => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_BANLISTFULL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOPRIVILEGES => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_CHANOPRIVSNEEDED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_CANTKILLSERVER => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_RESTRICTED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_UNIQOPPRIVSNEEDED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOOPERHOST => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_UMODEUNKNOWNFLAG => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_USERSDONTMATCH => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOPRIVS => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_MONLISTFULL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_METADATALIMIT => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_TARGETINVALID => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NOMATCHINGKEY => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_KEYINVALID => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_KEYNOTSET => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_KEYNOPERMISSION => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_NICKLOCKED => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_SASLFAIL => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_SASLTOOLONG => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_SASLABORT => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
            Response::ERR_SASLALREADY => (self.listener)(Event::Misc(String::from("name"), args, suffix)),
        }
    }
}

impl Channel {
    pub fn new() -> Self {
        Channel {
            name: String::new(),
            topic: String::new(),
            users: HashSet::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_users(&mut self, text: &str) {
        self.users.extend(text.split(" ").map(|s| s.to_string()));
    }

    pub fn get_users(&self) -> HashSet<String> {
        self.users.clone()
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    // #[test]
    // fn motd() {
    //     let mut s = Server::new();
    //     s.add_motd("Things and stuff".to_string());
    //     assert!(s.get_motd() == "Things and stuff\n".to_string());
    // }

    // #[test]
    // fn users() {
    //     let mut s = Server::new();
    //     let mut ch = Channel::new();
    //     s.add_users("one two three four".to_string());
    //     let mut target: HashSet<String> = HashSet::new();
    //     target.insert("one".to_string());
    //     target.insert("two".to_string());
    //     target.insert("three".to_string());
    //     target.insert("four".to_string());
    //     assert!(s.get_users() == target);
    // }

    // #[test]
    // fn channels() {
        
    // }
}