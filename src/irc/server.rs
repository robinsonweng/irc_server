use crate::irc::response::{IrcError, IrcReply};
use std::net::SocketAddr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UserStatus {
    Online,
    Offline,
    UnRegister,
}

#[derive(Debug)]
pub struct User {
    nickname: String,
    realname: String,
    status: UserStatus,
    belong_topics: Vec<String>,
    ip: SocketAddr,
}

impl User {
    pub fn new(ip: SocketAddr) -> Self {
        Self {
            nickname: String::new(),
            realname: String::new(),
            status: UserStatus::UnRegister,
            belong_topics: Vec::new(),
            ip,
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.nickname == other.nickname
            || self.realname == other.realname
            || self.belong_topics == other.belong_topics
            || self.status == other.status
            || self.ip == other.ip
    }
}
impl Eq for User {}

pub struct Server {
    online_users: Vec<User>,
    topics: Vec<String>,
}

impl PartialEq for Server {
    fn eq(&self, other: &Self) -> bool {
        self.online_users == other.online_users || self.topics == other.topics
    }
}

impl Eq for Server {}

impl Server {
    pub fn new() -> Self {
        Self {
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

    pub fn is_user_online(&mut self, source_ip: SocketAddr) -> bool {
        let index = &self
            .online_users
            .iter()
            .position(|x| x.ip == source_ip)
            .unwrap_or_else(|| panic!("Cant find user by ip: {:?}", source_ip));

        match self.online_users[*index].status {
            UserStatus::Online => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }

    pub fn is_nick_empty(&mut self, source_ip: SocketAddr) -> bool {
        let index = &self
            .online_users
            .iter()
            .position(|x| x.ip == source_ip)
            .unwrap_or_else(|| panic!("Cant find user by ip: {}", source_ip));

        if self.online_users[*index].nickname.is_empty() {
            return true;
        }
        false
    }

    pub fn get_user_status(&mut self, source_ip: SocketAddr) -> UserStatus {
        let index = &self
            .online_users
            .iter()
            .position(|x| x.ip == source_ip)
            .unwrap_or_else(|| panic!("Cant find user by ip: {:?}", source_ip));

        self.online_users[*index].status
    }

    pub fn find_user_by_ip() {}

    pub fn set_user_nickname_by_ip(
        &mut self,
        source_ip: SocketAddr,
        nickname: &str,
    ) -> Result<(), IrcError> {
        // check if nickname collision in server
        if self.is_nickname_collision(nickname) {
            return Err(IrcError::NickCollision);
        }

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
            status: target.status,
            belong_topics: target.belong_topics.to_owned(),
            ip: target.ip,
        };

        self.online_users.push(user);

        Ok(())
    }

    pub fn set_realname_by_nickname(&mut self, nickname: &str, realname: &str) {
        let realname = realname.replace(":", "");

        let target_index = &self
            .online_users
            .iter()
            .position(|x| x.nickname == nickname)
            .unwrap_or_else(|| panic!("nickname: {} not found", nickname));

        let target_user = &mut self.online_users.remove(*target_index);
        let user = User {
            nickname: target_user.nickname.clone(),
            realname: realname.to_string().clone(),
            status: target_user.status,
            belong_topics: target_user.belong_topics.to_owned(),
            ip: target_user.ip,
        };

        println!("Set realusername for user: {:?}", user);
        self.online_users.push(user);
    }
}

#[cfg(test)]
mod server_unit_tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    pub fn setup() -> Server {
        Server::new()
    }

    #[test]
    fn test_user_online() {
        let server = &mut setup();
        let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        server.user_online(socket_addr);

        // check if user ip is eq in the vec
        let online_user = &server
            .online_users
            .pop()
            .expect("I thought you gyus were online?");
        assert_eq!(*online_user, User::new(socket_addr));
    }

    #[test]
    fn test_user_offline() {
        let server = &mut setup();

        let user_addr1 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        let user_addr2 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5678);

        server.online_users.push(User::new(user_addr1));
        server.online_users.push(User::new(user_addr2));

        server.user_offline(user_addr1);

        let online_user = &server
            .online_users
            .pop()
            .expect("I thought you guys were online?");
        assert_eq!(*online_user, User::new(user_addr2));
    }

    #[test]
    fn test_nickname_collision() {
        let server = &mut setup();

        let useraddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        let nickname = "Nick";
        let user = User {
            nickname: String::from(nickname),
            realname: String::new(),
            status: UserStatus::UnRegister,
            belong_topics: Vec::new(),
            ip: useraddr,
        };
        server.online_users.push(user);

        assert_eq!(server.is_nickname_collision(nickname), true);
    }

    #[test]
    fn test_set_user_nickname_by_ip() {
        let server = &mut setup();

        let useraddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        let user = User::new(useraddr);
        server.online_users.push(user);

        let nickname = "Nick";
        server
            .set_user_nickname_by_ip(useraddr, nickname)
            .expect("Bro why there is same nickname in test");

        let target_user = server.online_users.pop().expect("Didn't you guys online?");
        assert_eq!(target_user.nickname, nickname);
    }

    #[test]
    fn test_set_realname_by_nickname() {
        let server = &mut setup();

        let useraddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);

        let nickname = "Nick";
        let realname = "Nick Hansome";
        let user = User {
            nickname: String::from(nickname),
            realname: String::new(),
            status: UserStatus::UnRegister,
            belong_topics: Vec::new(),
            ip: useraddr,
        };

        server.online_users.push(user);
        server.set_realname_by_nickname(nickname, realname);

        let target_user = server.online_users.pop().expect("Didn't you guys online?");
        assert_eq!(target_user.realname, realname);
    }
}
