use crate::irc::response::{IrcError, IrcReply};
use crate::irc::server::Server;
use std::net::SocketAddr;

#[derive(Debug)]
pub enum Command {
    SetNickName,
    User,
    Ping,
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

#[derive(Debug)]
pub struct CommandHandler {
    pub command: Command,
    pub context: String,
}

impl CommandHandler {
    pub fn new(raw_command: &str, raw_context: &str) -> Self {
        let command = match raw_command {
            "NICK" => Command::SetNickName,
            "USER" => Command::User,
            "PING" => Command::Ping,
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

    pub fn set_nickname(
        &self,
        server: &mut Server,
        nickname: &str,
        source_ip: SocketAddr,
    ) -> Result<(), IrcError> {
        // check if nickname collision in server
        if server.is_nickname_collision(nickname) {
            return Err(IrcError::NickCollision);
        }

        // set nickname by finding user using ip
        server.set_user_nickname_by_ip(source_ip, nickname);
        Ok(())
    }

    pub fn set_realname(&self, server: &mut Server, context: &str) -> Result<(), IrcError> {
        // parse context, (<nickname> <hostname> <servername> :<realname>)
        let msg = context.to_string().clone();
        let msg_parameter = msg.split_whitespace().collect::<Vec<&str>>();

        // ignore hostname & servername for now
        let (nickname, hostname, servername, realname) = (
            msg_parameter[0],
            msg_parameter[1],
            msg_parameter[2],
            msg_parameter[3],
        );

        if realname.is_empty() {
            todo!()
        }
        if !realname.starts_with(":") {
            todo!()
        }

        let parsed_realname = realname.replace(":", "");
        server.set_realname_by_nickname(nickname, &parsed_realname);

        Ok(())
    }

    fn ping(&self) {
        todo!()
    }

    fn list_channel(&self) {
        // list all channel, if the channel parameter is used, show the channel status only
        todo!()
    }
    fn join_topic(&self) {
        // client start listening a specific channel
        todo!()
    }

    fn change_topic() {
        // Change the channel topic in a mode +t channel
        todo!()
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
