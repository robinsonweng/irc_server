use std::io::Read;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;


const READ_TIMEOUT: (u64, u32) = (20, 0);
const WRITE_TIMEOUT: (u64, u32) = (20, 0);
const HOST_NAME: &'static str = "localhost";


fn handle_event(tcp_stream: TcpStream) -> std::io::Result<()> {
    let mut stream = tcp_stream;
    let client_ip = stream.peer_addr()?;
    let host_ip = stream.local_addr()?;

    let (r_second, r_micro_second) = READ_TIMEOUT;
    let (w_second, w_micro_second) = WRITE_TIMEOUT;
    stream.set_read_timeout(Some(Duration::new(r_second, r_micro_second)))?;
    stream.set_write_timeout(Some(Duration::new(w_second, w_micro_second)))?;


    loop {
        let mut buf: [u8; 128] = [0; 128];
        let raw_message = stream.read(&mut buf);

        let message = String::from_utf8((&buf).to_vec()).unwrap_or_else(|_| {
            panic!(
                "Cant convert message {:?} to utf-8 from client: {}",
                &raw_message, client_ip,
            )
        });

        break;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_ip = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 6667);

    let listener = TcpListener::bind(socket_ip)?;
    for stream in listener.incoming() {
        handle_event(stream?)?;
    }

    Ok(())
}
