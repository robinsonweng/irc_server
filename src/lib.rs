use std::net::{SocketAddr};

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
    // custom
    CommandNotFound
} 

pub struct User {
    nickname: String,
    belong_topics: Vec<String>,
    ip: SocketAddr,
}

impl User {
    pub fn new(ip: SocketAddr) -> User {
        User {
            nickname: String::new(),
            belong_topics: Vec::new(),
            ip,
        }
    }
}

pub struct Server <'a>{
    online_users: Vec<&'a mut User>,
    topics: Vec<String>,
}

impl <'a> Server <'a> {
    pub fn new() -> Server <'a>{
        Server {
            online_users: Vec::new(),
            topics: Vec::new(),
        }
    }

    pub fn user_online(&mut self, user: &'a mut User) {
        // self.online_users.contains(user) 
        self.online_users.push(user);
    }

    pub fn is_nickname_collision(&self, nickname: &str) -> bool {
        for user in &self.online_users {
            if nickname == user.nickname {
                return true;
            }
        }
        false
    }

}

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
            &_ => Command::CommandNotFound,
        };

        // what if message has multiple parameter
        let context = raw_context
            .clone()
            .to_string()
            .replace("\r\n", "");
        CommandHandler { 
            command,
            context,
        }
    }

    pub fn set_nickname(&self, user: &mut User) {
        // check if nickname collision in server
        // set nickname to user
        user.nickname = self.context.clone();
        println!("{:?}", self.context);
    }

    fn new_user(&self) {
        todo!()
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

