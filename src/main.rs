use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;

// from modules
mod irc;
use irc::command::{Command, CommandHandler};
use irc::response::{IrcError, IrcReply};
use irc::server::Server;

const READ_TIMEOUT: (u64, u32) = (10, 0);
const WRITE_TIMEOUT: (u64, u32) = (10, 0);

fn handle_event(tcp_stream: &TcpStream, server: &mut Server) -> std::io::Result<()> {
    let mut stream = tcp_stream;
    let source_ip = stream.peer_addr()?;

    let (r_second, r_micro_second) = READ_TIMEOUT;
    let (w_second, w_micro_second) = WRITE_TIMEOUT;
    stream.set_read_timeout(Some(Duration::new(r_second, r_micro_second)))?;
    stream.set_write_timeout(Some(Duration::new(w_second, w_micro_second)))?;

    server.user_online(source_ip);

    // tcp_stream.peek(&mut buf).expect("peak failed");

    loop {
        let mut buf: [u8; 128] = [0; 128];
        let result = stream.read(&mut buf);

        let message = String::from_utf8((&buf).to_vec()).unwrap_or_else(|_| {
            panic!(
                "Cant convert message {:?} to utf-8 from client: {}",
                &result, source_ip,
            )
        });

        println!(
            "{}",
            message.to_string().trim_matches(char::from(0)).clone()
        );

        let splited = message.trim_matches(char::from(0)).split_once(' ');

        let command_tuple: (&str, &str);
        if let Some(text) = splited {
            command_tuple = text;
        } else {
            break;
        }

        let (raw_command, raw_context) = command_tuple;
        let handler = CommandHandler::new(raw_command, raw_context);

        // if user not in server list, create new one

        match handler.command {
            Command::Capability => {
                // handle this later, since this is not in rfc, ref: https://ircv3.net/specs/extensions/capability-negotiation.html
            }
            Command::SetNickName => {
                // Introducing new nick or change exist nickname
                let nickname = &handler.context;
                match handler.set_nickname(server, &nickname, source_ip) {
                    Ok(()) => {
                        println!("Set ip: {} as nickname: {}", source_ip, &nickname);
                    }
                    Err(IrcError::NickCollision) => {
                        let nick_collision = format!("<nick> :Nickname collision {}", nickname);
                        stream.write(nick_collision.as_bytes())?;
                    }
                    _ => {}
                }
            }
            Command::User => {
                let context = &handler.context;
                match handler.set_realname(server, context) {
                    Ok(()) => {},
                    Err(IrcError::NeedMoreParams) => todo!(),
                    _ => {}
                }
            }
            _ => println!("Command: {} not found", raw_command),
        }
    }
    server.user_offline(source_ip);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_ip = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6667);

    let listener = TcpListener::bind(socket_ip)?;
    let mut server = Server::new();
    for stream in listener.incoming() {
        let client = stream?;
        handle_event(&client, &mut server)?;
    }

    Ok(())
}
