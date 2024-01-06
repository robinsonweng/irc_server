use crate::irc::response::{IrcError, IrcErrors, IrcReply, IrcResponseToMessage};
use crate::irc::server::{IrcServer, UserStatus, Server};
use regex::Regex;
use std::io::Write;
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

#[derive(Debug)]
pub struct CommandHandler {
    command: Command,
    context: String,
}

impl CommandHandler {
    pub fn new(raw_message: &str) -> Self {
        let (raw_command, raw_context) = Self::parse_message(raw_message);
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

        let context = raw_context.replace("\r\n", "");
        Self { command, context }
    }

    fn parse_message(raw_message: &str) -> (&str, &str) {
        let splited = raw_message.trim_matches(char::from(0)).split_once(' ');

        let command_tuple: (&str, &str);
        if let Some(text) = splited {
            command_tuple = text;
        } else {
            // not enouth args
            todo!()
        }
        return command_tuple;
    }

    pub fn execute(
        &self,
        stream: &mut TcpStream,
        server: &mut IrcServer,
        client_ip: SocketAddr,
    ) -> Result<(), IrcErrors> {
        // if first user detected in NICK command, wait untill user occor
        // unless timeout or offline
        // create user here
        // both username & realname should set, the user then can register as online

        match self.command {
            Command::SetNickName => {
                println!("this is NICK command");

                // do parsing
                let nickname = self.context.clone().replace("\r\n", "");

                let set_nickname_result = self.set_nickname(server, &nickname, client_ip);
                let msg = match set_nickname_result {
                    Ok(()) => None,
                    Err(IrcError::NickCollision) => {
                        Some(format!("{} :Nickname collision KILL\r\n", nickname))
                    }
                    // ERR_NONICKNAMEGIVEN
                    _ => panic!("set nickname didn't match any rpl or err"),
                };

                if msg.is_some() {
                    // ERR_NICKCOLLISION
                    stream.write(msg.unwrap().as_bytes())?;
                }
            }

            Command::User => {
                println!("this is USER command");
                println!("command: {:?}", self.command);
                println!("context: {:?}", self.context);

                let context = self.context.replace("\r\n", "");

                // mabye use regex instead
                let context_collection: Vec<&str> = context.split(' ').collect();
                if let [username, hostname, servername, raw_realname] = &context_collection[..] {
                    if !raw_realname.starts_with(":") {
                        // ERR_NEEDMOREPARAMS
                        let need_more_param = IrcError::NeedMoreParams.to_message(
                            "USER",
                            "",
                            "Username should start with prefix ':'.",
                        );
                        stream.write(need_more_param.as_bytes())?;
                        return Ok(());
                    }

                    let realname = &raw_realname.replace(":", "");
                    println!(
                        "username: {}, hostname: {}, servername: {}, realname: {}",
                        username, hostname, servername, realname
                    );
                    self.set_username(server, client_ip, username);
                    self.set_realname(server, client_ip, realname);
                    // handle hostname & server name later
                } else {
                    // ERR_NEEDMOREPARAMS
                    let need_more_param = IrcError::NeedMoreParams.to_message(
                        "USER",
                        "",
                        "Username should start with prefix ':'.",
                    );
                    stream.write(need_more_param.as_bytes())?;
                    return Ok(());
                }
            }

            Command::Ping => {
                // ERR_NOORIGIN if no server is given
                println!("this is PING command");

                let context = self.context.replace("\r\n", "");
                if context.is_empty() {
                    // find nick from ip
                    let nickname = "";
                    let hostname = "localhost";
                    let no_origin = IrcError::NoOrigin.to_message(
                        hostname,
                        nickname,
                        ":No origin specified\r\n",
                    );
                    stream.write(no_origin.as_bytes())?;
                    return Ok(());
                }
            }

            Command::Join => {
                /*
                    user joins a channel
                    if channel dont exist, create one
                    the channel ceases to exist if last user leave
                    regular channel (channel name start with '#') are all known on every server
                    local channel (channel name start with '&') are only known on one spcific server
                */
                println!("this is JOIN command");
                let context = self.context.replace("\r\n", "");

                let channel_regex = Regex::new(r"^JOIN (#\w+)([,]#\w+)?( \w+)?([,]\w+)?").unwrap();
                if channel_regex.is_match(&context) {
                    println!("correct format: {}", context);
                } else {
                    println!("incorrect formfromat: {}", context);
                }
            }

            Command::List => {
                todo!()
            }
            Command::Topic => {
                // ERR_NOORIGIN
                // ERR_NOSUCHSERVER
                println!("this is PONG command");
            }
            Command::Pong => {
                todo!()
            }
            _ => {
                println!("Command not support: {:?}", self.command);
            }
        }
        Ok(())
    }

    pub fn user_online(
        &self,
        stream: &mut TcpStream,
        server: &mut IrcServer,
        hostname: &str,
        client_ip: SocketAddr,
    ) -> std::io::Result<()> {
        // is username & nickname is set & status == unregister
        // Start wellcome message here

        let hostname = hostname.to_string().clone();

        let nickname = self.is_user_ready_to_register(server, client_ip);
        if nickname.is_none() {
            return Ok(());
        }
        let nickname = nickname.unwrap();

        let welcome =
            IrcReply::Welcome.to_message(&hostname, &nickname, ":Welcome to the rust irc server");
        stream.write(welcome.as_bytes())?;

        let yourhost = IrcReply::YourHost.to_message(
            &hostname,
            &nickname,
            ":Your host is rust irc, running version 1.0.0-dev",
        );
        stream.write(yourhost.as_bytes())?;

        let created =
            IrcReply::Created.to_message(&hostname, &nickname, "Are you solid like a rock?");
        stream.write(created.as_bytes())?;

        let myinfo = IrcReply::MyInfo.to_message(&hostname, &nickname, "Let me see see");
        stream.write(myinfo.as_bytes())?;

        self.set_user_status(server, client_ip, UserStatus::Online);
        Ok(())
    }

    pub fn set_nickname(
        &self,
        server: &mut IrcServer,
        nickname: &str,
        source_ip: SocketAddr,
    ) -> Result<(), IrcError> {
        // set nickname by finding user using ip
        let result = server.set_user_nickname_by_ip(source_ip, nickname);
        result
    }

    pub fn set_realname(&self, server: &mut IrcServer, source_ip: SocketAddr, realname: &str) {
        server.set_realname_by_ip(source_ip, realname);
    }

    pub fn set_username(&self, server: &mut IrcServer, source_ip: SocketAddr, username: &str) {
        server.set_username_by_ip(source_ip, username);
    }

    pub fn set_user_status(&self, server: &mut IrcServer, source_ip: SocketAddr, status: UserStatus) {
        server.set_user_status_by_ip(source_ip, status);
    }

    pub fn is_user_ready_to_register(
        &self,
        server: &mut IrcServer,
        source_ip: SocketAddr,
    ) -> Option<String> {
        if !server.is_user_ready_to_register(source_ip) {
            return None;
        }
        Some(server.get_user_nick(source_ip))
    }
}
