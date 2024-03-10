use std::fmt::Write as _;


use irc_server::irc::execute;


mod common;
use common::{MockedStream, MockedUser};


const HOST_ADDR: &str = "localhost";
const USER_NAME: &str = "rob";
const NICK_NAME: &str = "haru";
const CHANNEL_NAME: &str = "rust";


fn setup() -> (MockedStream, MockedUser) {
    let mocked_stream = MockedStream {
        read_message: Vec::new(),
        write_message: Vec::new(),
    };
    let mocked_user = MockedUser {
        username: String::new(),
        nickname: String::new(),
    };
    (mocked_stream, mocked_user)
}

#[test]
fn test_command_capability() {
    /*
        we dont support CAP command now
    */
    let (mut stream, mut user) = setup();

    stream.read_message.push(b"CAP LS 302".to_vec());

    let _ = execute(&mut stream, &mut user);

    let result = stream.write_message.pop();

    assert!(result.is_none());
}


#[test]
fn test_command_user_when_nick_is_not_set() {
    // if nick is not set, return nothing,
    let (mut stream, mut user) = setup();

    let mut request_message = String::new();
    let _ = write!(&mut request_message, "USER {}\r\n", USER_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    let _ = execute(&mut stream, &mut user);

    let raw_response = stream.write_message.pop();

    assert!(raw_response.is_none());

    assert_eq!(user.username, USER_NAME);
}


#[test]
fn test_command_nick_when_username_is_not_set() {
    let (mut stream, mut user) = setup();

    let request_message = format!("NICK {}\r\n", NICK_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    let _ = execute(&mut stream, &mut user);

    assert_eq!(user.nickname, NICK_NAME);

    let response = stream.write_message.pop();

    assert!(response.is_none());
}


#[test]
fn test_command_user_when_nick_is_set() {
    // if nick is also set, return 001, 002, 003, 004
    let (mut stream, mut user) = setup();

    user.nickname = NICK_NAME.to_string();

    let request_message = format!("USER {}\r\n", USER_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    let _ = execute(&mut stream, &mut user);

    assert_eq!(user.username, USER_NAME);
    assert_eq!(user.nickname, NICK_NAME);

    let raw_welcome = stream.write_message.pop();

    assert!(!raw_welcome.is_none());

    let welcome = raw_welcome.unwrap();

    let expected_welcome_msg = format!(
        "{} 001 :Welcome to the rust irc server\r\n",
        HOST_ADDR,
    );
    assert_eq!(welcome, expected_welcome_msg.as_bytes().to_vec());

}
