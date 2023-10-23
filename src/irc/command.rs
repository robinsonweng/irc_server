use crate::irc::response::{IrcError, IrcReply};
use crate::irc::server::{Server, UserStatus};
use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub enum Command {
    SetNickName,
    User,
    Ping,
    Pong,
    List,
    Join,
    Topic,
    Names,
    Part,
    Users,
    PrivateMessage,
    Quit,
    Capability,
    // custom
    CommandNotFound,
}

pub struct CommandParser {
    pub command: Command,
    pub context: String,
}

impl CommandParser {
    pub fn new(raw_command: &str, raw_context: &str) -> Self {
        let command = match raw_command {
            "NICK" => Command::SetNickName,
            "USER" => Command::User,
            "PING" => Command::Ping,
            "PONG" => Command::Pong,
            "LIST" => Command::List,
            "JOIN" => Command::Join,
            "TOPIC" => Command::Topic,
            "NAMES" => Command::Names,
            "PART" => Command::Part,
            "USERS" => Command::Users,
            "PRIVMSG" => Command::PrivateMessage,
            "QUIT" => Command::Quit,
            "CAP" => Command::Capability,
            &_ => Command::CommandNotFound,
        };

        // what if message has multiple parameter
        let context = raw_context.clone().to_string().replace("\r\n", "");
        Self { command, context }
    }
}

#[derive(Debug)]
pub struct CommandHandler {}

impl CommandHandler {
    pub fn set_nickname(
        &self,
        server: &mut Server,
        nickname: &str,
        source_ip: SocketAddr,
    ) -> Result<(), IrcError> {
        // set nickname by finding user using ip
        let result = server.set_user_nickname_by_ip(source_ip, nickname);
        result
    }

    pub fn set_realname(&self, server: &mut Server, source_ip: SocketAddr, realname: &str) {
        server.set_realname_by_ip(source_ip, realname);
    }

    pub fn set_username(&self, server: &mut Server, source_ip: SocketAddr, username: &str) {
        server.set_username_by_ip(source_ip, username);
    }

    pub fn set_user_status(&self, server: &mut Server, source_ip: SocketAddr, status: UserStatus) {
        server.set_user_status_by_ip(source_ip, status);
    }

    pub fn is_user_ready_to_register(
        &self,
        server: &mut Server,
        source_ip: SocketAddr,
    ) -> Option<String> {
        if !server.is_user_ready_to_register(source_ip) {
            return None;
        }
        Some(server.get_user_nick(source_ip))
    }

    pub fn ping(&self, server: &mut Server, source_ip: SocketAddr) -> String {
        todo!()
    }

    pub fn wellcome(stream: TcpStream) {}
}
