pub enum IrcReply {
    Users = 393,
}

impl IrcReply {
    pub fn from_usize(&self, status: usize) -> IrcReply {
        match status {
            393 => IrcReply::Users,
            _ => panic!("IRC Reply status code not found"),
        }
    }
}

pub enum IrcError {
    NickCollision = 436,
    NeedMoreParams = 461,
    AlreadyRegistred = 462,
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
