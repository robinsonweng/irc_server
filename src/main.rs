use std::net::{TcpListener, Ipv4Addr, SocketAddrV4, TcpStream};
use std::time::Duration;
use std::io::{Read, Write};

// from lib
use irc_server::{Server, User, Command, CommandHandler};

const READ_TIMEOUT: (u64, u32) = (10, 0);
const WRITE_TIMEOUT: (u64, u32) = (10, 0);


fn handle_event(tcp_stream: TcpStream, server: &mut Server) -> std::io::Result<()> {
    let mut stream = tcp_stream;
    let source_ip = stream.peer_addr()?;
    println!("user joined via ip:port {:?}", source_ip);

    let (r_second, r_micro_second) = READ_TIMEOUT;
    let (w_second, w_micro_second) = WRITE_TIMEOUT;
    stream.set_read_timeout(Some(Duration::new(r_second, r_micro_second)))?;
    stream.set_write_timeout(Some(Duration::new(w_second, w_micro_second)))?;

    // tcp_stream.peek(&mut buf).expect("peak failed");

    let mut buf: [u8; 128] = [0; 128];
    let result = stream.read(&mut buf);

    let message = String::from_utf8((&buf).to_vec())
        .unwrap_or_else(|_| panic!("Cant convert message {:?} to utf-8 from client: {}", &result, source_ip));


    let (raw_command, raw_context) = message.trim_matches(char::from(0)).split_once(' ').unwrap();

    dbg!(raw_command);
    dbg!(raw_context);

    // find user by ip, if not found, create user

    let handler = CommandHandler::new(raw_command, raw_context);
    match handler.command {
        Command::SetNickName => {
            server.is_nickname_collision(&handler.context);
        },
        _ => !todo!()
    }
    Ok(())

}

fn main() -> std::io::Result<()> {
    let socket_ip = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6667);

    let listener = TcpListener::bind(socket_ip)?;
    let server = &mut Server::new();
    for stream in listener.incoming() {
        handle_event(stream?, server)?;
    }

    Ok(())
}
