use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;

// from modules
mod irc;
use irc::command::{Command, CommandHandler, CommandParser};
use irc::response::{IrcError, IrcReply, IrcResponse};
use irc::server::{Server, UserStatus};

const READ_TIMEOUT: (u64, u32) = (20, 0);
const WRITE_TIMEOUT: (u64, u32) = (20, 0);
const HOST_NAME: &'static str = "localhost";

fn handle_event(tcp_stream: TcpStream, server: &mut Server) -> std::io::Result<()> {
    let mut stream = tcp_stream;
    let client_ip = stream.peer_addr()?;
    let host_ip = stream.local_addr()?;

    let (r_second, r_micro_second) = READ_TIMEOUT;
    let (w_second, w_micro_second) = WRITE_TIMEOUT;
    stream.set_read_timeout(Some(Duration::new(r_second, r_micro_second)))?;
    stream.set_write_timeout(Some(Duration::new(w_second, w_micro_second)))?;

    server.user_online(client_ip);

    loop {
        let mut buf: [u8; 128] = [0; 128];
        let raw_message = stream.read(&mut buf);

        let message = String::from_utf8((&buf).to_vec()).unwrap_or_else(|_| {
            panic!(
                "Cant convert message {:?} to utf-8 from client: {}",
                &raw_message, client_ip,
            )
        });

        let splited = message.trim_matches(char::from(0)).split_once(' ');

        let command_tuple: (&str, &str);
        if let Some(text) = splited {
            command_tuple = text;
        } else {
            break;
        }

        let (raw_command, raw_context) = command_tuple;
        let parser = CommandParser::new(raw_command, raw_context);

        let handler = CommandHandler {};

        // if first user detected in NICK command, wait untill user occor
        // unless timeout or offline
        // create user here
        // both username & realname should set, the user then can register as online

        // ALSO, you don't struct Server, since this code only accept single thread currently
        // the struct User should handle single thread scenario perfectly

        match parser.command {
            Command::SetNickName => {
                println!("this is NICK command");
                println!("raw command: {}", raw_command);
                println!("raw context: {}", raw_context);

                let nickname = parser.context.clone();
                let set_nick_result = handler.set_nickname(server, &nickname, client_ip);
                let msg = match set_nick_result {
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
                    continue;
                }
            }

            Command::User => {
                println!("this is USER command");
                println!("command raw: {}", raw_command);
                println!("raw context: {}", raw_context.clone());

                let context = raw_context.clone().replace("\r\n", "");
                let context_collection = context.split(' ').collect::<Vec<&str>>();
                if let [username, hostname, servername, raw_realname] = &context_collection[..] {
                    if !raw_realname.starts_with(":") {
                        // ERR_NEEDMOREPARAMS
                        let need_more_param = IrcError::NeedMoreParams.to_message(
                            "USER",
                            "",
                            "Username should start with prefix ':'.",
                        );
                        stream.write(need_more_param.as_bytes())?;
                        continue;
                    }

                    let realname = &raw_realname.replace(":", "");
                    println!(
                        "username: {}, hostname: {}, servername: {}, realname: {}",
                        username, hostname, servername, realname
                    );

                    handler.set_username(server, client_ip, username);
                    handler.set_realname(server, client_ip, realname);
                    // handle hostname & servername later
                } else {
                    // ERR_NEEDMOREPARAMS
                    let need_more_param =
                        IrcError::NeedMoreParams.to_message("USER", "", "Not enough parameters");
                    stream.write(need_more_param.as_bytes())?;
                    continue;
                }
            }

            Command::Ping => {
                // ERR_NOORIGIN  if no server is given
                println!("this is Ping command");

                if raw_context.is_empty() {
                    // find nick from ip
                    let nickname = "";
                    let no_origin = IrcError::NoOrigin.to_message(HOST_NAME, &nickname, ":No origin specified\r\n");
                    stream.write(no_origin.as_bytes())?;
                    continue;
                }
                println!("the pinged server is: {}", raw_context);
                stream.write("Pong\r\n".as_bytes())?;
            }

            Command::Pong => {
                // ERR_NOORIGIN
                // ERR_NOSUCHSERVER
                println!("this is Pong command");
            }
            _ => println!("Command: {} not found", raw_command),
        }

        // is username & nickname is set & status == unregister
        // Start wellcome message here

        let nickname = handler.is_user_ready_to_register(server, client_ip);
        if nickname.is_none() {
            continue;
        }
        let nickname = nickname.unwrap();

        let welcome =
            IrcReply::Welcome.to_message(HOST_NAME, &nickname, ":Welcome to the rust irc server");
        stream.write(welcome.as_bytes())?;

        let yourhost = IrcReply::YourHost.to_message(
            HOST_NAME,
            &nickname,
            ":Your host is rust irc, running version 1.0.0-dev",
        );
        stream.write(yourhost.as_bytes())?;

        let created =
            IrcReply::Created.to_message(HOST_NAME, &nickname, "Are you solid like a rock?");
        stream.write(created.as_bytes())?;

        let myinfo = IrcReply::MyInfo.to_message(HOST_NAME, &nickname, "Let me see see");
        stream.write(myinfo.as_bytes())?;

        handler.set_user_status(server, client_ip, UserStatus::Online);
    }

    // notice: user timeout or offline
    server.user_offline(client_ip);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_ip = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6667);

    let listener = TcpListener::bind(socket_ip)?;
    let mut server = Server::new();
    for stream in listener.incoming() {
        let client = stream?;
        handle_event(client, &mut server)?;
    }

    Ok(())
}
