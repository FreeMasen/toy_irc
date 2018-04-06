use std::collections::HashSet;

pub struct Server {
    motd: String,
    users: HashSet<String>,
    channels: HashSet<String>,

}

impl Server {

    pub fn new() -> Server {
        Server {
            motd: String::new(),
            users: HashSet::new(),
            channels: HashSet::new(),
        }
    }

    pub fn add_motd(&mut self, text: String) {
        self.motd += if text.ends_with("\n") {
            &text
        } else {
            &(text + "\n")
        };
    }

    pub fn get_motd(&self) -> String {
        self.motd
    }

    pub fn add_users(&mut self, text: String) {
        let new_users: HashSet<String> = text.split(" ").map(|s| s.to_string()).collect();
        self.users.extend(&new_users);
    }

    pub fn get_users(&self) -> HashSet<String> {
        self.users
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn motd() {
        let mut s = Server::new();
        s.add_motd("Things and stuff".to_string());
        assert!(s.get_motd() == "Things and stuff\n".to_string());
    }

    #[test]
    fn users() {
        let mut s = Server::new();
        s.add_users("one two three four".to_string());
        let mut target: HashSet<String> = HashSet::new();
        target.insert("one".to_string());
        target.insert("two".to_string());
        target.insert("three".to_string());
        target.insert("four".to_string());
        assert!(s.get_users() == target);
    }
}