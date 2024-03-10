mod common;
use common::MockedStream;

// user name: rob
// channel name: rust

const USER_NAME: &str = "rob";
const CHANNEL_NAME: &str = "rust";


/*
#[test]
fn main_test() {
    let mut mocked_stream = MockedStream {
        read_message: Vec::new(),
        write_message: Vec::new(),
    };

    mocked_stream.read_message.push(b"123 123".to_vec());

    let _ = execute(&mut mocked_stream);

    let result = mocked_stream.write_message.pop().unwrap();

    assert_eq!(result, b"cool\r\n".to_vec());
} 
*/

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

    let result = stream.write_message.pop();

    assert!(result.is_none());
}
