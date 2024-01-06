use std::io::Read;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;

// from modules
mod irc;
use irc::command::CommandHandler;
use irc::response::IrcErrors;
use irc::server::{IrcServer, Server};

const READ_TIMEOUT: (u64, u32) = (20, 0);
const WRITE_TIMEOUT: (u64, u32) = (20, 0);
const HOST_NAME: &'static str = "localhost";

fn handle_event(tcp_stream: TcpStream, server: &mut IrcServer) -> std::io::Result<()> {
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

        let command_handler = CommandHandler::new(&message);

        // should handle error from execute, other wise the user will only online
        let result = command_handler.execute(&mut stream, server, client_ip);
        match result {
            Ok(()) => {}
            Err(IrcErrors::BadMessage) => {
                continue;
            }
            Err(IrcErrors::BadStream) => {
                break;
            }
        }

        // should handle error from here
        command_handler.user_online(&mut stream, server, HOST_NAME, client_ip)?;
    }

    // mabye add notice: user timeout or offline
    // only trigger if loop breaked
    server.user_offline(client_ip);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_ip = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6667);

    let listener = TcpListener::bind(socket_ip)?;
    let mut server = IrcServer::new();
    for stream in listener.incoming() {
        let client = stream?;
        handle_event(client, &mut server)?;
    }

    Ok(())
}
