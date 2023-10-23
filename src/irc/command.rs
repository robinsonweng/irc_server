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

    fn list_all_nicknames() {
        //  By using the NAMES command, a user can list all nicknames that are
        // visible to them on any channel that they can see.
        todo!()
    }

    fn leave_channel() {
        // The PART message causes the client sending the message to be removed
        // from the list of active users for all given channels listed in the
        // parameter string.
        todo!()
    }

    fn list_users() {
        // The USERS command returns a list of users logged into the server in a
        // similar  format
        todo!()
    }

    fn private_message() {
        //  PRIVMSG is used to send private messages between users.  <receiver>
        // is the nickname of the receiver of the message.  <receiver> can also
        // be a list of names or channels separated with commas.
        todo!()
    }

    fn quit() {
        // A client session is ended with a quit message.  The server must close
        // the connection to a client which sends a QUIT message.
        todo!()
    }
}
