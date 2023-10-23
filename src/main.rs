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
    let source_ip = stream.peer_addr()?;
    let host_ip = stream.local_addr()?;

    let (r_second, r_micro_second) = READ_TIMEOUT;
    let (w_second, w_micro_second) = WRITE_TIMEOUT;
    stream.set_read_timeout(Some(Duration::new(r_second, r_micro_second)))?;
    stream.set_write_timeout(Some(Duration::new(w_second, w_micro_second)))?;

    server.user_online(source_ip);

    loop {
        let mut buf: [u8; 128] = [0; 128];
        let raw_message = stream.read(&mut buf);

        let message = String::from_utf8((&buf).to_vec()).unwrap_or_else(|_| {
            panic!(
                "Cant convert message {:?} to utf-8 from client: {}",
                &raw_message, source_ip,
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

                let nickname = &parser.context;
                let set_nick_result = handler.set_nickname(server, nickname, source_ip);
                let msg = match set_nick_result {
                    Ok(()) => None,
                    Err(IrcError::NickCollision) => {
                        Some(format!("{} :Nickname collision KILL", nickname))
                    }
                    _ => panic!("set nickname didn't match any rpl or err"),
                };

                if msg.is_some() {
                    // return error message
                    break;
                }

                let wellcome = format!(
                    ":{} 001 robinson :Welcome to the WeeChat IRC server\r\n",
                    HOST_NAME
                );
                stream.write(wellcome.as_bytes())?;

                let yourhost = format!(
                    ":{} 002 robinson :Your host is weercd, running version 1.0.0-dev\r\n",
                    HOST_NAME
                );
                stream.write(yourhost.as_bytes())?;

                let created = format!(
                    ":{} 003 robinson :Are you solid like a rock?\r\n",
                    HOST_NAME
                );
                stream.write(created.as_bytes())?;

                let myinfo = format!(":{} 004 robinson :Let's see!\r\n", HOST_NAME);
                stream.write(myinfo.as_bytes())?;
            }

            Command::User => {
                println!("this is USER command");
                // nick should first
                if handler.is_nick_empty(server, source_ip) {
                    println!("nick is empty");
                    break;
                }

                // if !handler.is_user_online(server, source_ip) {
                //     break;
                // }
                handler.set_user_status(server, source_ip, UserStatus::Online);
                // ERR_NEEDMOREPARAMS
                // ERR_ALREADYREGISTRED
            }

            Command::Pong => {}
            _ => println!("Command: {} not found", raw_command),
        }
        // if no response from ping, server view user as offline
        // if handler.is_user_online(server, source_ip) {
        //     let ping_msg = handler.ping(server, source_ip);
        //     stream.write(ping_msg.as_bytes())?;

        // ping from server
        // let ping_msg = format!("PING :{}\r\n", HOST_NAME);
        // stream.write(ping_msg.as_bytes())?;
    }

    // notice: user timeout or offline
    server.user_offline(source_ip);

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
