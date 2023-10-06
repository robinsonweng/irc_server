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

impl IrcReply {
    pub fn from_usize(&self, status: usize) -> IrcReply {
        match status {
            393 => IrcReply::Users,
            _ => panic!("IRC Reply status code not found"),
        }
    }
}

#[derive(Debug)]
pub enum IrcError {
    NoSuchNick = 401,
    NoSuchChannel = 403,
    NoRecipent = 411,
    NoTextTosend = 412,
    NoNicknameGiven = 431,
    NickCollision = 436,
    NeedMoreParams = 461,
    AlreadyRegistred = 462,
    BadRequest = 800,
    InternalError = 900,
}

impl IrcError {
    pub fn from_usize(&self, status: usize) -> IrcError {
        match status {
            436 => IrcError::NickCollision,
            461 => IrcError::NeedMoreParams,
            462 => IrcError::AlreadyRegistred,
            _ => panic!("IRC Errorstatus code not found"),
        }
    }
}
