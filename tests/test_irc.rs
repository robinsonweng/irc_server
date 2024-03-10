use std::fmt::Write as _;


use irc_server::irc::execute;


mod common;
use common::MockedStream;


const USER_NAME: &str = "rob";
const CHANNEL_NAME: &str = "rust";


fn setup() -> MockedStream {
    MockedStream {
        read_message: Vec::new(),
        write_message: Vec::new(),
    }
}

#[test]
fn test_command_capability() {
    /*
        we dont support CAP command now
    */
    let mut stream = setup();

    stream.read_message.push(b"CAP LS 302".to_vec());

    let _ = execute(&mut stream);

    let result = stream.write_message.pop();

    assert!(result.is_none());
}


#[test]
fn test_command_user_when_nick_is_not_set() {
    // if nick is not set, return nothing,
    let mut stream = setup();

    let mut request_message = String::new();
    let _ = write!(&mut request_message, "USER {}", USER_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    let _ = execute(&mut stream);

    let raw_response = stream.write_message.pop();

    assert!(raw_response.is_none());
}
