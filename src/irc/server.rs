use std::net::SocketAddr;

#[derive(Debug)]
pub struct User {
    nickname: String,
    realname: String,
    belong_topics: Vec<String>,
    ip: SocketAddr,
}

impl User {
    pub fn new(ip: SocketAddr) -> User {
        User {
            nickname: String::new(),
            realname: String::new(),
            belong_topics: Vec::new(),
            ip,
        }
    }
}

pub struct Server {
    online_users: Vec<User>,
    topics: Vec<String>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            online_users: Vec::new(),
            topics: Vec::new(),
        }
    }

    pub fn user_online(&mut self, source_ip: SocketAddr) {
        // self.online_users.contains(user)
        let user = User::new(source_ip);
        println!("User: {:?} online!", user);
        self.online_users.push(user);
    }

    pub fn user_offline(&mut self, source_ip: SocketAddr) {
        let index = self
            .online_users
            .iter()
            .position(|x| x.ip == source_ip)
            .unwrap();

        println!("User: {:?} offline!", self.online_users[index]);
        self.online_users.remove(index);
    }

    pub fn is_nickname_collision(&self, nickname: &str) -> bool {
        for user in &self.online_users {
            if nickname.to_string() == user.nickname {
                return true;
            }
        }
        false
    }

    pub fn set_user_nickname_by_ip(&mut self, source_ip: SocketAddr, nickname: &str) {
        let name = nickname.to_string().clone();
        let index = &self
            .online_users
            .iter()
            .position(|x| x.ip == source_ip)
            .unwrap_or_else(|| panic!("Cant find user by ip: {:?}", source_ip));

        let target = &mut self.online_users.remove(*index);
        let user = User {
            nickname: name,
            realname: target.realname.clone(),
            belong_topics: target.belong_topics.to_owned(),
            ip: target.ip,
        };
        self.online_users.push(user);
    }
}
