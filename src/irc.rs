use std::io::{Read, Write};


#[derive(PartialEq, Eq)]
enum CommandFromUser {
    CAP,
    USER,
    NICK,
}

impl From<String> for CommandFromUser {
    fn from(command: String) -> Self {
        match command.as_str() {
            "CAP" => Self::CAP,
            "USER" => Self::USER,
            "NICK" => Self::NICK,
            _=> todo!(),
        }
    }
}


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

    let (raw_command, message_with_newline) = splited_message.unwrap();

    let message = message_with_newline.replace("\r\n", "");

    let command = CommandFromUser::from(raw_command.to_string());
    match command{
        CommandFromUser::CAP => return Ok(()),
        CommandFromUser::USER => user.set_username(&message),
        CommandFromUser::NICK => user.set_nickname(&message),
    };

    if user.register_complete() && (command == CommandFromUser::USER || command == CommandFromUser::NICK){
        let welcome = format!("{} 001 :Welcome to the rust irc server\r\n", "localhost");
        let _ = stream.write(welcome.as_bytes())?;

        return Ok(());
    }

    println!("incoming command: '{}'", raw_command);
    println!("incoming message: '{}'", message);

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
