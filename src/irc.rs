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
        // return welcome message
        let (
            welcome,
            your_host,
            created,
            my_info
        ) = welcome_messages("localhost", user.get_nickname());

        let _ = stream.write(welcome.as_bytes())?;
        let _ = stream.write(your_host.as_bytes())?;
        let _ = stream.write(created.as_bytes())?;
        let _ = stream.write(my_info.as_bytes())?;

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
    fn get_nickname(&self) -> &str;
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
    fn get_nickname(&self) -> &str {
        &self.nickname
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

pub fn welcome_messages(host_name: &str, nickname: &str) -> (String, String, String, String) {
    let welcome = format!(
        "{} 001 :Welcome to the rust irc server\r\n",
        host_name,
    );
    let your_host = format!(
        "{} {} 002 :Your host is rust irc, running version 1.0.0-dev\r\n",
        host_name,
        nickname,
    );
    let created = format!(
        "{} {} 003 :Are you solid like a rock?\r\n",
        host_name,
        nickname,
    );
    let my_info = format!(
        "{} {} 004 :Let me see see\r\n",
        host_name,
        nickname,
    );

    (welcome, your_host, created, my_info)
}
