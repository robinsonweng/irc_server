use core::panic;
use std::io::{Read, Write};

use irc_server::irc::execute;

// user name: rob
// channel name: rust

const USER_NAME: &str = "rob";
const CHANNEL_NAME: &str = "rust";


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

struct MockedStream {
    read_message: Vec<Vec<u8>>,
    write_message: Vec<Vec<u8>>,
}

impl Read for MockedStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let raw_message = self.read_message.pop();
        if raw_message.is_none() {
            panic!("no bytes to read");
        }

        let message = raw_message.unwrap();
        let len = message.len();
        buf[..len].copy_from_slice(&message);

        Ok(len)
    }
}

impl Write for MockedStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let len = buf.len();
        self.write_message.push(buf.to_vec());
        Ok(len)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
