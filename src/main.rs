use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;

// from modules
mod irc;
use irc::command::{Command, CommandHandler, CommandParser};
use irc::response::{IrcError, IrcReply};
use irc::server::Server;

const READ_TIMEOUT: (u64, u32) = (20, 0);
const WRITE_TIMEOUT: (u64, u32) = (20, 0);
const HOST_NAME: &'static str = "coool";

fn msg_prefix(numeric: u8, nickname: &str, msg: &str) -> String {
    format!(":{} {} {} {}\r\n", HOST_NAME, numeric, nickname, msg)
}

fn handle_event(tcp_stream: &TcpStream, server: &mut Server) -> std::io::Result<()> {
    let mut stream = tcp_stream;
    let source_ip = stream.peer_addr()?;
    let host_ip = stream.local_addr()?;

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
        let parser = CommandParser::new(raw_command, raw_context);

        let handler = CommandHandler {};

        // if user not in server list, create new one

        match parser.command {
            Command::Capability => {
                // handle this later, since this is not in rfc, ref: https://ircv3.net/specs/extensions/capability-negotiation.html
            }
            Command::SetNickName => {
                // Introducing new nick or change exist nickname
                let nickname = &parser.context;
                let is_newbie = server.is_new_user(source_ip);
                let error_msg = handler.set_nickname(server, nickname, source_ip);

                if !is_newbie {
                    if error_msg.is_some() {
                        stream.write(error_msg.unwrap().as_bytes())?;
                    } else {
                        break;
                    }
                }

                // rfc 1459 8.5 Establishing a server to client connection
                // 1. send MOTD
                // 2. LUSER
                // 3. server return its name & version
                // 4. USER
                // 5. NICK with dns information?
                let motd_start = format!(":- {:?} Message of the day - ", HOST_NAME);
                let motd_msg = format!(":- yoyo three to the one");
                let motd_end = ":End of /MOTD command".to_string();

                stream.write(
                    &msg_prefix(IrcReply::MOTDStart as u8, nickname, &motd_start).as_bytes(),
                )?;
                stream.write(&msg_prefix(IrcReply::MOTD as u8, nickname, &motd_msg).as_bytes())?;
                stream.write(
                    &msg_prefix(IrcReply::EndOfMOTD as u8, nickname, &motd_end).as_bytes(),
                )?;
            }

            Command::User => {
                let context = &parser.context;
                match handler.set_realname(server, context) {
                    Ok(()) => {}
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
