#[derive(Debug, Clone, Copy)]
pub enum IrcReply {
    Welcome = 1,
    YourHost = 2,
    Created = 3,
    MyInfo = 4,
    IsSupport = 5,

    ListStart = 321,
    List = 322,
    Listend = 323,

    NoTopic = 331,
    Topic = 332,
    NamReply = 353,
    EndOfNames = 366,

    MOTD = 372,
    MOTDStart = 375,
    EndOfMOTD = 376,

    UsersStart = 392,
    Users = 393,
    EndOfUsers = 394,
}

#[derive(Debug, Clone, Copy)]
pub enum IrcError {
    NoSuchNick = 401,
    NoSuchChannel = 403,
    NoRecipent = 411,
    NoTextTosend = 412,
    NoNicknameGiven = 431,
    NickCollision = 436,
    NeedMoreParams = 461,
    AlreadyRegistred = 462,
}

pub trait IrcResponse {
    fn to_message(&self, hostname: &str, nickname: &str, body: &str) -> String {
        todo!()
    }
}

impl IrcResponse for IrcReply {
    fn to_message(&self, hostname: &str, nickname: &str, body: &str) -> String {
        format!(":{} {:3} {} {}\r\n", hostname, *self as u16, nickname, body)
    }
}
impl IrcResponse for IrcError {
    fn to_message(&self, hostname: &str, nickname: &str, body: &str) -> String {
        format!(":{} {:3} {} {}\r\n", hostname, *self as u16, nickname, body)
    }
}
