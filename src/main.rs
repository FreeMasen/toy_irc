mod server;
// mod irc_client;

// use irc_client::*;
extern crate irc;
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
    reactor.register_client_with_handler(client, |_client, message| {
        
        handle_message(message);
        Ok(())
    });
    reactor.run().expect("Unable to run reactor");
}

fn handle_message(message: Message) {
    // if let Some(tags) = message.tags {
    //     let tags: Vec<String> = tags.into_iter().map(|t| format!("{:?}", t)).collect();
    //     println!("tags: {}", tags.join(" "));
    // }
    // if let Some(prefix) = message.prefix {
    //     println!("prefix:{}", prefix.replace("\n", " "));
    // }
    match message.command {
        Command::PASS(pwd) => println!("PASS {}", pwd),
        Command::NICK(name) => println!("NICK {}", name),
        Command::USER(user, mode, realname) => println!("USER {}, {}, {}", user, mode, realname),
        Command::OPER(name, pwd) => println!("OPER {} {}", name, pwd),
        Command::UserMODE(mode, nics) => {
            let nicks: Vec<String> = nics.into_iter().map(|m| format!("{:?}", m)).collect();
            println!("UserMODE {}{:?}", mode, nicks.join(""));
        },
        Command::SERVICE(service, nic, reserved, dist, tp, res_info,) => println!("SERVICE {}, {}, {}, {}, {}, {}", service, nic, reserved, dist, tp, res_info),
        Command::QUIT(comment) => println!("QUIT {:?}", comment),
        Command::SQUIT(server, comment) => println!("SQUIT {}, {:?}", server, comment),
        Command::JOIN(list, keys, realname) => println!("JOIN {}, {:?}, {:?}", list, keys, realname),
        Command::PART(list, comment) => println!("PART {}, {:?}", list, comment),
        Command::ChannelMODE(channel, modes) => {
            let modes: Vec<String> = modes.into_iter().map(|m| format!("{:?}", m)).collect();
            println!("ChannelMODE {},{}", channel, modes.join(""));
        },
        Command::TOPIC(channel, topic) => println!("TOPIC {}, {:?}", channel, topic),
        Command::NAMES(list, target) => println!("NAMES {:?}, {:?}", list, target),
        Command::LIST(list, target) => println!("LIST {:?}, {:?}", list, target),
        Command::INVITE(nickname, channel) => println!("INVITE {:?}, {:?}",nickname, channel),
        Command::KICK(list, user_list, comment) => println!("KICK {}, {}, {:?}", list, user_list, comment),
        Command::PRIVMSG(target, text) => print_message(message.prefix, target, text),
        Command::NOTICE(target, text) => println!("NOTICE {} {}", target, text),
        Command::MOTD(target) => println!("MOTD {:?}", target),
        Command::LUSERS(mask, target) => println!("LUSERS {:?}, {:?}", mask, target),
        Command::VERSION(version) => println!("VERSION {:?}", version),
        Command::STATS(query, target) => println!("STATS {:?}, {:?}", query, target),
        Command::LINKS(server, mask) => println!("LINKS {:?}, {:?}", server, mask),
        Command::TIME(time) => println!("TIME {:?}", time),
        Command::CONNECT(server, port, remote) => println!("CONNECT {:}:{:}, {:?}", server, port, remote),
        Command::TRACE(target) => println!("TRACE {:?}", target),
        Command::ADMIN(target) => println!("ADMIN {:?}", target),
        Command::INFO(target) => println!("INFO {:?}", target),
        Command::SERVLIST(mask, tp) => println!("SERVLIST {:?}, {:?}", mask, tp),
        Command::SQUERY(name, text) => println!("SQUERY {}, {}", name, text),
        Command::WHO(mask, operator) => println!("WHO {:?}, {:?}", mask, operator),
        Command::WHOIS(target, list) => println!("WHOIS {:?}  {:?}", target, list),
        Command::WHOWAS(list, count, target) => println!("WHOWAS {}, {:?}, {:?}", list, count, target),
        Command::KILL(name, comment) => println!("KILL {}, {}", name, comment),
        Command::PING(me, you) => println!("PING {}, {:?}", me, you),
        Command::PONG(me, you) => println!("PONG {}, {:?}", me, you),
        Command::ERROR(msg) => println!("ERROR {}", msg),
        Command::AWAY(msg) => println!("AWAY {:?}", msg),
        Command::REHASH => println!("REHASH"),
        Command::DIE => println!("DIE"),
        Command::RESTART => println!("RESTART"),
        Command::SUMMON(user, target, channel) => println!("SUMMON {}, {:?}, {:?}", user, target, channel),
        Command::USERS(list) => println!("USERS {:?}", list),
        Command::WALLOPS(text) => println!("WALLOPS {}", text),
        Command::USERHOST(list) => println!("USERHOST {}", list.join(", ")),
        Command::ISON(list) => println!("ISON {}", list.join(" ")),
        Command::SAJOIN(name, channel) => println!("SAJOIN {}, {}", name, channel),
        Command::SAMODE(target, modes, params) => println!("SAMODE {}, {}, {:?}", target, modes, params),
        Command::SANICK(old, new) => println!("SANICK {}, {}", old, new),
        Command::SAPART(name, comment) => println!("SAPART {} {}", name, comment),
        Command::SAQUIT(name, comment) => println!("SAQUIT {}, {}", name, comment),
        Command::NICKSERV(msg) => println!("NICKSERV {}", msg),
        Command::CHANSERV(msg) => println!("CHANSERV {}", msg),
        Command::OPERSERV(msg) => println!("OPERSERV {}", msg),
        Command::BOTSERV(msg) => println!("BOTSERV {}", msg),
        Command::HOSTSERV(msg) => println!("HOSTSERV {}", msg),
        Command::MEMOSERV(msg) => println!("MEMOSERV {}", msg),
        Command::CAP(cmd, sub_cmd, arg, param) => println!("CAP {:?}, {:?}. {:?}. {:?}", cmd, sub_cmd, arg, param),
        Command::AUTHENTICATE(name) => println!("AUTHENTICATE {}", name),
        Command::ACCOUNT(name) => println!("ACCOUNT {}", name),
        Command::METADATA(target, sub_cmd, params, param) => {
            let params = if let Some(p) = params {
                p.join(", ")
            } else {
                String::new()
            };
            println!("METADATA {} {:?}, {}, {:?}", target, sub_cmd, params, param);
        },
        Command::MONITOR(command, list) => println!("MONITOR {}, {:?}", command, list),
        Command::BATCH(operator, sub_cmd, params) => {
            let params = if let Some(params) = params {
                params.join(", ")
            } else {
                String::new()
            };
            println!("BATCH {} {:?}, {:?}", operator, sub_cmd, params);
        },
        Command::CHGHOST(user, host) => println!("CHGHOST {}, {}", user, host),
        Command::Response(res, args, suffix) => response(res, args, suffix) ,
        Command::Raw(command, params, param) => println!("Raw {}, {}, {:?}", command, params.join(", "), param),
    }
}

fn print_message(prefix: Option<String>, channel: String, text: String) {
    
    let user_name = match prefix {
        Some(p) => {
            let parts: Vec<&str> = p.split('!').collect();
            String::from(parts[0])
        },
        None => String::from("Unknown")
    };
    println!("{} @{}$: {}", channel, user_name, text);
}

fn response(res: Response, args: Vec<String>, suffix: Option<String>) {
    println!("Response");
    match res {
        Response::RPL_WELCOME => println!("RPL_WELCOME"),
        Response::RPL_YOURHOST => println!("RPL_YOURHOST"),
        Response::RPL_CREATED => println!("RPL_CREATED"),
        Response::RPL_MYINFO => println!("RPL_MYINFO"),
        Response::RPL_ISUPPORT => println!("RPL_ISUPPORT"),
        Response::RPL_BOUNCE => println!("RPL_BOUNCE"),
        Response::RPL_NONE => println!("RPL_NONE"),
        Response::RPL_USERHOST => println!("RPL_USERHOST"),
        Response::RPL_ISON => println!("RPL_ISON"),
        Response::RPL_AWAY => println!("RPL_AWAY"),
        Response::RPL_UNAWAY => println!("RPL_UNAWAY"),
        Response::RPL_NOWAWAY => println!("RPL_NOWAWAY"),
        Response::RPL_WHOISUSER => println!("RPL_WHOISUSER"),
        Response::RPL_WHOISSERVER => println!("RPL_WHOISSERVER"),
        Response::RPL_WHOISOPERATOR => println!("RPL_WHOISOPERATOR"),
        Response::RPL_WHOISIDLE => println!("RPL_WHOISIDLE"),
        Response::RPL_ENDOFWHOIS => println!("RPL_ENDOFWHOIS"),
        Response::RPL_WHOISCHANNELS => println!("RPL_WHOISCHANNELS"),
        Response::RPL_WHOWASUSER => println!("RPL_WHOWASUSER"),
        Response::RPL_ENDOFWHOWAS => println!("RPL_ENDOFWHOWAS"),
        Response::RPL_LISTSTART => println!("RPL_LISTSTART"),
        Response::RPL_LIST => println!("RPL_LIST"),
        Response::RPL_LISTEND => println!("RPL_LISTEND"),
        Response::RPL_UNIQOPIS => println!("RPL_UNIQOPIS"),
        Response::RPL_CHANNELMODEIS => println!("RPL_CHANNELMODEIS"),
        Response::RPL_NOTOPIC => println!("RPL_NOTOPIC"),
        Response::RPL_TOPIC => println!("RPL_TOPIC"),
        Response::RPL_TOPICWHOTIME => println!("RPL_TOPICWHOTIME"),
        Response::RPL_INVITING => println!("RPL_INVITING"),
        Response::RPL_SUMMONING => println!("RPL_SUMMONING"),
        Response::RPL_INVITELIST => println!("RPL_INVITELIST"),
        Response::RPL_ENDOFINVITELIST => println!("RPL_ENDOFINVITELIST"),
        Response::RPL_EXCEPTLIST => println!("RPL_EXCEPTLIST"),
        Response::RPL_ENDOFEXCEPTLIST => println!("RPL_ENDOFEXCEPTLIST"),
        Response::RPL_VERSION => println!("RPL_VERSION"),
        Response::RPL_WHOREPLY => println!("RPL_WHOREPLY"),
        Response::RPL_ENDOFWHO => println!("RPL_ENDOFWHO"),
        Response::RPL_NAMREPLY => println!("RPL_NAMREPLY"),
        Response::RPL_ENDOFNAMES => println!("RPL_ENDOFNAMES"),
        Response::RPL_LINKS => println!("RPL_LINKS"),
        Response::RPL_ENDOFLINKS => println!("RPL_ENDOFLINKS"),
        Response::RPL_BANLIST => println!("RPL_BANLIST"),
        Response::RPL_ENDOFBANLIST => println!("RPL_ENDOFBANLIST"),
        Response::RPL_INFO => println!("RPL_INFO"),
        Response::RPL_ENDOFINFO => println!("RPL_ENDOFINFO"),
        Response::RPL_MOTDSTART => println!("RPL_MOTDSTART"),
        Response::RPL_MOTD => println!("RPL_MOTD"),
        Response::RPL_ENDOFMOTD => println!("RPL_ENDOFMOTD"),
        Response::RPL_YOUREOPER => println!("RPL_YOUREOPER"),
        Response::RPL_REHASHING => println!("RPL_REHASHING"),
        Response::RPL_YOURESERVICE => println!("RPL_YOURESERVICE"),
        Response::RPL_TIME => println!("RPL_TIME"),
        Response::RPL_USERSSTART => println!("RPL_USERSSTART"),
        Response::RPL_USERS => println!("RPL_USERS"),
        Response::RPL_ENDOFUSERS => println!("RPL_ENDOFUSERS"),
        Response::RPL_NOUSERS => println!("RPL_NOUSERS"),
        Response::RPL_HOSTHIDDEN => println!("RPL_HOSTHIDDEN"),
        Response::RPL_TRACELINK => println!("RPL_TRACELINK"),
        Response::RPL_TRACECONNECTING => println!("RPL_TRACECONNECTING"),
        Response::RPL_TRACEHANDSHAKE => println!("RPL_TRACEHANDSHAKE"),
        Response::RPL_TRACEUKNOWN => println!("RPL_TRACEUKNOWN"),
        Response::RPL_TRACEOPERATOR => println!("RPL_TRACEOPERATOR"),
        Response::RPL_TRACEUSER => println!("RPL_TRACEUSER"),
        Response::RPL_TRACESERVER => println!("RPL_TRACESERVER"),
        Response::RPL_TRACESERVICE => println!("RPL_TRACESERVICE"),
        Response::RPL_TRACENEWTYPE => println!("RPL_TRACENEWTYPE"),
        Response::RPL_TRACECLASS => println!("RPL_TRACECLASS"),
        Response::RPL_TRACERECONNECT => println!("RPL_TRACERECONNECT"),
        Response::RPL_TRACELOG => println!("RPL_TRACELOG"),
        Response::RPL_TRACEEND => println!("RPL_TRACEEND"),
        Response::RPL_STATSLINKINFO => println!("RPL_STATSLINKINFO"),
        Response::RPL_STATSCOMMANDS => println!("RPL_STATSCOMMANDS"),
        Response::RPL_ENDOFSTATS => println!("RPL_ENDOFSTATS"),
        Response::RPL_STATSUPTIME => println!("RPL_STATSUPTIME"),
        Response::RPL_STATSOLINE => println!("RPL_STATSOLINE"),
        Response::RPL_UMODEIS => println!("RPL_UMODEIS"),
        Response::RPL_SERVLIST => println!("RPL_SERVLIST"),
        Response::RPL_SERVLISTEND => println!("RPL_SERVLISTEND"),
        Response::RPL_LUSERCLIENT => println!("RPL_LUSERCLIENT"),
        Response::RPL_LUSEROP => println!("RPL_LUSEROP"),
        Response::RPL_LUSERUNKNOWN => println!("RPL_LUSERUNKNOWN"),
        Response::RPL_LUSERCHANNELS => println!("RPL_LUSERCHANNELS"),
        Response::RPL_LUSERME => println!("RPL_LUSERME"),
        Response::RPL_ADMINME => println!("RPL_ADMINME"),
        Response::RPL_ADMINLOC1 => println!("name"),
        Response::RPL_ADMINLOC2 => println!("name"),
        Response::RPL_ADMINEMAIL => println!("RPL_ADMINEMAIL"),
        Response::RPL_TRYAGAIN => println!("RPL_TRYAGAIN"),
        Response::RPL_LOCALUSERS => println!("RPL_LOCALUSERS"),
        Response::RPL_GLOBALUSERS => println!("RPL_GLOBALUSERS"),
        Response::RPL_WHOISCERTFP => println!("RPL_WHOISCERTFP"),
        Response::RPL_MONONLINE => println!("RPL_MONONLINE"),
        Response::RPL_MONOFFLINE => println!("RPL_MONOFFLINE"),
        Response::RPL_MONLIST => println!("RPL_MONLIST"),
        Response::RPL_ENDOFMONLIST => println!("RPL_ENDOFMONLIST"),
        Response::RPL_WHOISKEYVALUE => println!("RPL_WHOISKEYVALUE"),
        Response::RPL_KEYVALUE => println!("RPL_KEYVALUE"),
        Response::RPL_METADATAEND => println!("RPL_METADATAEND"),
        Response::RPL_LOGGEDIN => println!("RPL_LOGGEDIN"),
        Response::RPL_LOGGEDOUT => println!("RPL_LOGGEDOUT"),
        Response::RPL_SASLSUCCESS => println!("RPL_SASLSUCCESS"),
        Response::RPL_SASLMECHS => println!("RPL_SASLMECHS"),
        Response::ERR_UNKNOWNERROR => println!("name"),
        Response::ERR_NOSUCHNICK => println!("name"),
        Response::ERR_NOSUCHSERVER => println!("name"),
        Response::ERR_NOSUCHCHANNEL => println!("name"),
        Response::ERR_CANNOTSENDTOCHAN => println!("name"),
        Response::ERR_TOOMANYCHANNELS => println!("name"),
        Response::ERR_WASNOSUCHNICK => println!("name"),
        Response::ERR_TOOMANYTARGETS => println!("name"),
        Response::ERR_NOSUCHSERVICE => println!("name"),
        Response::ERR_NOORIGIN => println!("name"),
        Response::ERR_NORECIPIENT => println!("name"),
        Response::ERR_NOTEXTTOSEND => println!("name"),
        Response::ERR_NOTOPLEVEL => println!("name"),
        Response::ERR_WILDTOPLEVEL => println!("name"),
        Response::ERR_BADMASK => println!("name"),
        Response::ERR_UNKNOWNCOMMAND => println!("name"),
        Response::ERR_NOMOTD => println!("name"),
        Response::ERR_NOADMININFO => println!("name"),
        Response::ERR_FILEERROR => println!("name"),
        Response::ERR_NONICKNAMEGIVEN => println!("name"),
        Response::ERR_ERRONEOUSNICKNAME => println!("name"),
        Response::ERR_NICKNAMEINUSE => println!("name"),
        Response::ERR_NICKCOLLISION => println!("name"),
        Response::ERR_UNAVAILRESOURCE => println!("name"),
        Response::ERR_USERNOTINCHANNEL => println!("name"),
        Response::ERR_NOTONCHANNEL => println!("name"),
        Response::ERR_USERONCHANNEL => println!("name"),
        Response::ERR_NOLOGIN => println!("name"),
        Response::ERR_SUMMONDISABLED => println!("name"),
        Response::ERR_USERSDISABLED => println!("name"),
        Response::ERR_NOTREGISTERED => println!("name"),
        Response::ERR_NEEDMOREPARAMS => println!("name"),
        Response::ERR_ALREADYREGISTRED => println!("name"),
        Response::ERR_NOPERMFORHOST => println!("name"),
        Response::ERR_PASSWDMISMATCH => println!("name"),
        Response::ERR_YOUREBANNEDCREEP => println!("name"),
        Response::ERR_YOUWILLBEBANNED => println!("name"),
        Response::ERR_KEYSET => println!("name"),
        Response::ERR_CHANNELISFULL => println!("name"),
        Response::ERR_UNKNOWNMODE => println!("name"),
        Response::ERR_INVITEONLYCHAN => println!("name"),
        Response::ERR_BANNEDFROMCHAN => println!("name"),
        Response::ERR_BADCHANNELKEY => println!("name"),
        Response::ERR_BADCHANMASK => println!("name"),
        Response::ERR_NOCHANMODES => println!("name"),
        Response::ERR_BANLISTFULL => println!("name"),
        Response::ERR_NOPRIVILEGES => println!("name"),
        Response::ERR_CHANOPRIVSNEEDED => println!("name"),
        Response::ERR_CANTKILLSERVER => println!("name"),
        Response::ERR_RESTRICTED => println!("name"),
        Response::ERR_UNIQOPPRIVSNEEDED => println!("name"),
        Response::ERR_NOOPERHOST => println!("name"),
        Response::ERR_UMODEUNKNOWNFLAG => println!("name"),
        Response::ERR_USERSDONTMATCH => println!("name"),
        Response::ERR_NOPRIVS => println!("name"),
        Response::ERR_MONLISTFULL => println!("name"),
        Response::ERR_METADATALIMIT => println!("name"),
        Response::ERR_TARGETINVALID => println!("name"),
        Response::ERR_NOMATCHINGKEY => println!("name"),
        Response::ERR_KEYINVALID => println!("name"),
        Response::ERR_KEYNOTSET => println!("name"),
        Response::ERR_KEYNOPERMISSION => println!("name"),
        Response::ERR_NICKLOCKED => println!("name"),
        Response::ERR_SASLFAIL => println!("name"),
        Response::ERR_SASLTOOLONG => println!("name"),
        Response::ERR_SASLABORT => println!("name"),
        Response::ERR_SASLALREADY => println!("name"),
    }

    println!("Args:{}", args.join(""));
    if let Some(suffix) = suffix {
        println!("Suffix:{}", suffix);
    }
}