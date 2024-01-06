#[cfg(test)]
mod command_tests {
    use std::net::{SocketAddr, IpAddr, Ipv4Addr, TcpStream};
    use irc_server::irc::command::CommandHandler;
    use irc_server::irc::server::{Server, IrcServer};

    struct MockServer {

    }

    pub fn setup(raw_message: &str) -> CommandHandler {
        CommandHandler::new(raw_message)
    }

    #[test]
    fn test_execute_command_new_nickname() {
        let raw_message = "NICK Wiz";
        let client_ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        let mut server = IrcServer::new();
        let mut stream: TcpStream;

        let command_handler = setup(raw_message);
        // command_handler.execute(&mut stream, &mut server, client_ip);
    }

    #[test]
    fn test_execute_command_change_nickname() {
        let raw_message = ":WiZ NICK Kilroy";
        let client_ip = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);
        let mut server = IrcServer::new();
        let mut stream: TcpStream;

        let command_handler = setup(raw_message);
    }
}
