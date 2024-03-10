use std::io::{Read, Write};


pub fn execute<T>(stream: &mut T, user: &mut impl User) -> std::io::Result<()>
where T: Read + Write
{
    let mut buf: [u8; 128] = [0; 128];
    let _ = stream.read(&mut buf);

    let raw_message = String::from_utf8((&buf).to_vec()).unwrap_or_else(|_| {
        panic!(
            "Cant convert message {:?} to utf-8", &buf
        )
    });

    let splited_message = raw_message.trim_matches(char::from(0)).split_once(' ');
    if splited_message.is_none() {
        todo!()
    }

    let (command, message_with_newline) = splited_message.unwrap();

    let message = message_with_newline.replace("\r\n", "");

    if command == "CAP" {
        return Ok(());
    }

    if command == "USER" {
        user.set_username(&message);
        return Ok(());
    }

    if command == "NICK" {
        user.set_nickname(&message);
        return Ok(());
    }


    println!("incoming command: '{}'", command);
    println!("incoming message: '{}'", message);

    // let _ = stream_writer.write(b"cool\r\n");

    Ok(())  // command not found?
}


pub trait User {
    fn register_complete(&self) -> bool;
    fn set_username(&mut self, username: &str);
    fn set_nickname(&mut self, nickname: &str);
}

pub struct IrcUser {
    username: String,
    nickname: String,
}

impl User for IrcUser {
    fn register_complete(&self) -> bool {
        !self.nickname.is_empty() && !self.username.is_empty()
    }
    fn set_username(&mut self, username: &str) {
        self.username = username.to_string();
    }
    fn set_nickname(&mut self, nickname: &str) {
        self.nickname = nickname.to_string();
    }
}

impl IrcUser {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            nickname: String::new(),
        }
    }
}
