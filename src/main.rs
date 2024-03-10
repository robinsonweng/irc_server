use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;

use irc_server::irc::{execute, IrcUser};


const READ_TIMEOUT: (u64, u32) = (20, 0);
const WRITE_TIMEOUT: (u64, u32) = (20, 0);
//const HOST_NAME: &'static str = "localhost";


fn main() -> std::io::Result<()> {
    let socket_ip = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6667);

    let listener = TcpListener::bind(socket_ip)?;
    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }

    Ok(())
}


fn handle_connection(tcp_stream: TcpStream) -> std::io::Result<()> {
    let mut stream = tcp_stream;
    let client_ip = stream.peer_addr()?;
    // let host_ip = stream.local_addr()?;

    let (r_second, r_micro_second) = READ_TIMEOUT;
    let (w_second, w_micro_second) = WRITE_TIMEOUT;
    stream.set_read_timeout(Some(Duration::new(r_second, r_micro_second)))?;
    stream.set_write_timeout(Some(Duration::new(w_second, w_micro_second)))?;


    let mut user = IrcUser::new();
    execute(&mut stream, &mut user)?;

    Ok(())
}
