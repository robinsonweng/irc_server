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
    NoOrigin = 409,
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
        let msg = format!(
            ":{} {:0>3} {} :{}\r\n",
            hostname, *self as u16, nickname, body
        );
        println!("{}", msg.clone());
        msg
    }
}
impl IrcResponse for IrcError {
    fn to_message(&self, primary: &str, secondary: &str, body: &str) -> String {
        let msg: String;
        if !primary.is_empty() && !secondary.is_empty() {
            msg = format!("{} {} :{}\r\n", primary, secondary, body);
        } else if !primary.is_empty() && secondary.is_empty() {
            msg = format!("{} :{}\r\n", primary, body);
        } else {
            msg = format!(":{}\r\n", body);
        }

        println!("{}", msg.clone());
        msg
    }
}
