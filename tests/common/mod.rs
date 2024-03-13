use std::io::{Read, Write};

use irc_server::irc::User;


pub struct MockedStream {
    pub read_message: Vec<Vec<u8>>,
    pub write_message: Vec<Vec<u8>>,
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

impl MockedStream {
    pub fn expect_message(&mut self, expected_message: Option<&str>) {
        let message = self.write_message
            .pop()
            .map(|x| String::from_utf8(x).expect("incorrect message"));
        assert_eq!(message, Option::from(expected_message).map(|x: &str| x.to_string()));
    }
}

pub struct MockedUser {
    pub username: String,
    pub nickname: String,
    pub realname: String,
}

impl User for MockedUser {
    fn register_complete(&self) -> bool {
        !self.username.is_empty() && !self.nickname.is_empty()
    }
    fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }
    fn set_nickname(&mut self, nickname: &str) {
        self.nickname = nickname.to_string();
    }
    fn get_nickname(&self) -> &str {
        &self.nickname
    }
    fn set_real_name(&mut self, realname: &str) {
        self.realname = realname.to_string();
    }
}
