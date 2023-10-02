use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;
use std::time::Duration;

const READ_TIMEOUT: (u64, u32) = (5, 0);
const WRITE_TIMEOUT: (u64, u32) = (5, 0);

fn handle_event(tcp_stream: TcpStream, server: &mut Server) -> std::io::Result<()> {
    let mut stream = tcp_stream;
    let source_ip = stream.peer_addr()?;
    println!("user joined via ip:port {:?}", source_ip);

    let (r_second, r_micro_second) = READ_TIMEOUT;
    let (w_second, w_micro_second) = WRITE_TIMEOUT;
    stream.set_read_timeout(Some(Duration::new(r_second, r_micro_second)))?;
    stream.set_write_timeout(Some(Duration::new(w_second, w_micro_second)))?;

    // tcp_stream.peek(&mut buf).expect("peak failed");

    loop {
        let mut buf: [u8; 128] = [0; 128];
        let result = stream.read(&mut buf);

        let message = String::from_utf8((&buf).to_vec()).unwrap_or_else(|_| {
            panic!(
                "Cant convert message {:?} to utf-8 from client: {}",
                &result, user_ip
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
        dbg!(raw_command);
        dbg!(raw_context);

        let handler = CommandHandler::new(raw_command, raw_context);

        // if user not in server list, create new one
        let mut user = &mut User::new(user_ip);

        match handler.command {
            Command::Capability => {
                // handle this later, since this is not in rfc, ref: https://ircv3.net/specs/extensions/capability-negotiation.html
            }
            Command::SetNickName => {
                let target = server.find_user_by_ip(user_ip);
                if target.is_some() {
                    user = target.unwrap();
                }

                let nickname = raw_context.to_string();
                let collision = server.username_is_collision(&nickname);
                println!("Check if username collision or not: {}", collision);
                if collision == true {
                    // same nickname in user exist
                    break;
                }
                user.set_nickname(nickname);
            }
            Command::User => {
                // when user online, send both nickname & realname to server

                // server.user_online(user);
            }
            Command::CommandNotFound => println!("Command: {} not found", raw_command),
            _ => !todo!(),
        }
    }
    Ok(())
    // let handler = CommandHandler::new(raw_command, raw_context);

    // match handler.command {
    //     Command::SetNickName => {
    //         let user = User::new(handler.context, user_ip);
    //         server.online_users.push(user);
    //     }
    //     Command::Capability => {
    //         println!("cap cap");
    //         stream.write(b"123");
    //     }
    //     Command::CommandNotFound => {
    //         println!("What?");
    //     }
    //     _ => !todo!(),
    // }
}

fn main() -> std::io::Result<()> {
    let socket_ip = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6667);

    let listener = TcpListener::bind(socket_ip)?;
    let mut server = Server::new();
    for stream in listener.incoming() {
        handle_event(stream?, &mut server)?;
        println!("Client Disconnect");
    }

    Ok(())
}
