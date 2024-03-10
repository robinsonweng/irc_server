use std::io::{Read, Write};

use irc_server::irc::execute;



#[test]
fn main_test() {
    let mut mocked_stream = MockedStream {
        read_message: Vec::new(),
        write_message: Vec::new(),
    };

    let _ = execute(&mut mocked_stream);

    let result = mocked_stream.write_message.pop().unwrap();

    assert_eq!(result, b"cool".to_vec());
}

struct MockedStream {
    read_message: Vec<Vec<u8>>,
    write_message: Vec<Vec<u8>>,
}
impl Read for MockedStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let len = buf.len();
        self.read_message.push(buf.to_vec());
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
