use std::net::{TcpListener, Ipv4Addr, SocketAddrV4, TcpStream};
use std::task::Context;
use std::time::Duration;


const READ_TIMEOUT: (u64, u32) = (10, 0);
const WRITE_TIMEOUT: (u64, u32) = (10, 0);

struct CommandHandler {
    command: Command,
    context: String,
}

impl CommandHandler {
    fn new(raw_command: &str, raw_context: &str) -> Self {
        let command = match raw_command {
            "NICK" => Command::SetNickName,
            "USER" => Command::User,
            "PING" => Command::Ping,
            "LIST" => Command::List,
            "JOIN" => Command::Join,
            "TOPIC" => Command::Topic,
            "Names" => Command::Names,
            "PART" => Command::Part,
            "USERS" => Command::Users,
            "PRIVMSG" => Command::PrivateMessage,
            "QUIT" => Command::Quit,
            &_ => Command::CommandNotFound,
        };

        let context = raw_context.to_string();
        CommandHandler { 
            command,
            context,
        }
    }
}

enum Command {
    SetNickName,
    User,
    Ping,
    List,
    Join,
    Topic,
    Names,
    Part,
    Users,
    PrivateMessage,
    Quit,
    // custom
    CommandNotFound
} 


fn set_nickname(context: String) {

}

fn handle_event(tcp_stream: TcpStream) -> std::io::Result<()> {
    let user_ip = tcp_stream.peer_addr().unwrap();
    println!("user joined via ip:port {}", user_ip);

    let (r_second, r_micro_second) = READ_TIMEOUT;
    let (w_second, w_micro_second) = WRITE_TIMEOUT;
    tcp_stream.set_read_timeout(Some(Duration::new(r_second, r_micro_second)))?;
    tcp_stream.set_write_timeout(Some(Duration::new(w_second, w_micro_second)))?;

    let mut buf = [0; 128];
    tcp_stream.peek(&mut buf).expect("peak failed");

    let message = String::from_utf8((&buf).to_vec())
        .unwrap_or_else(|_| panic!("Cant convert message {:?} to utf-8 from client: {}", &buf, user_ip));


    let (raw_command, raw_context) = message.trim_matches(char::from(0)).split_once(' ').unwrap();

    dbg!(raw_command);
    dbg!(raw_context);

    let handler = CommandHandler::new(raw_command, raw_context);


    match handler.command {
        Command::SetNickName => {
            set_nickname(
                handler.context
            )
        },
        _ => !todo!()
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
