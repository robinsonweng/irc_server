use std::fmt::Write as _;


use irc_server::irc::{execute, welcome_messages};


mod common;
use common::{MockedStream, MockedUser};


const HOST_ADDR: &str = "localhost";
const USER_NAME: &str = "rob";
const NICK_NAME: &str = "haru";
const REAL_NAME: &str = "robinson weng";
const CHANNEL_NAME: &str = "rust";


fn setup() -> (MockedStream, MockedUser) {
    let mocked_stream = MockedStream {
        read_message: Vec::new(),
        write_message: Vec::new(),
    };
    let mocked_user = MockedUser {
        username: String::new(),
        nickname: String::new(),
        realname: String::new(),
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

    stream.expect_message(None);
}


#[test]
fn test_command_user_when_nick_is_not_set() {
    // if nick is not set, return nothing,
    let (mut stream, mut user) = setup();

    let mut request_message = String::new();
    let _ = write!(&mut request_message, "USER {} 0 * :{}\r\n", USER_NAME, REAL_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    let _ = execute(&mut stream, &mut user);

    stream.expect_message(None);

    assert_eq!(user.username, USER_NAME);
}


#[test]
fn test_command_user_when_nick_is_set() {
    // if nick is also set, return 001, 002, 003, 004
    let (mut stream, mut user) = setup();

    user.nickname = NICK_NAME.to_string();

    let request_message = format!("USER {} 0 * :{}\r\n", USER_NAME, REAL_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    let _ = execute(&mut stream, &mut user);

    assert_eq!(user.username, USER_NAME);
    assert_eq!(user.nickname, NICK_NAME);
    assert_eq!(user.realname, REAL_NAME);

    // since the pop will do FILO, the message order should be reverse
    let (
        expected_welcome,
        expected_your_host,
        expected_created,
        expected_my_info,
    ) = welcome_messages(HOST_ADDR, NICK_NAME);

    stream.expect_message(Some(&expected_my_info));

    stream.expect_message(Some(&expected_created));

    stream.expect_message(Some(&expected_your_host));

    stream.expect_message(Some(&expected_welcome));
}


#[test]
fn test_command_nick_when_username_is_not_set() {
    let (mut stream, mut user) = setup();

    let request_message = format!("NICK {}\r\n", NICK_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    let _ = execute(&mut stream, &mut user);

    assert_eq!(user.nickname, NICK_NAME);

    let _ = stream.expect_message(None);
}


#[test]
fn test_command_nick_when_username_is_set() {
    // if nick is also set, return 001, 002, 003, 004
    let (mut stream, mut user) = setup();

    user.username = USER_NAME.to_string();

    let request_message = format!("NICK {}\r\n", NICK_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    let _ = execute(&mut stream, &mut user);

    assert_eq!(user.username, USER_NAME);
    assert_eq!(user.nickname, NICK_NAME);

    let (
        expected_welcome,
        expected_your_host,
        expected_created,
        expected_my_info,
    ) = welcome_messages(HOST_ADDR, NICK_NAME);

    stream.expect_message(Some(&expected_my_info));

    stream.expect_message(Some(&expected_created));

    stream.expect_message(Some(&expected_your_host));

    stream.expect_message(Some(&expected_welcome));
}


#[test]
fn test_command_join() {
    // if nick is also set, return 001, 002, 003, 004
    let (mut stream, mut user) = setup();

    user.username = USER_NAME.to_string();

    let request_message = format!("JOIN {}\r\n", NICK_NAME);

    stream.read_message.push(request_message.as_bytes().to_vec());

    // let _ = execute(&mut stream, &mut user);

    // assert_eq!(user.username, USER_NAME);
    // assert_eq!(user.nickname, NICK_NAME);
}
