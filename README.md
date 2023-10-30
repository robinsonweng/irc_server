# A toy example for irc server in rust
This is a irc server implmentation in rust & [rfc1459](https://www.rfc-editor.org/rfc/rfc1459.html) & [modern irc protocol](https://modern.ircdocs.horse/).

## TODO

### Main feature
- [x] Use tcplisten to listen irc port (6667) 
- Implment most command in single thread
    - [x] NICK
    - [x] USER
    - [ ] PING
    - [ ] LIST
    - [ ] JOIN
    - [ ] TOPIC
    - [ ] NAMES
    - [ ] PART
    - [ ] USERS
    - [ ] PRIVMSG
    - [ ] QUIT

- Implment necessasary numeric commands
    - [x] (001) RPL_MYINFO 
    - [x] (002) RPL_CREATED 
    - [x] (003) RPL_YOURHOST 
    - [x] (004) RPL_WELCOME 
    - [x] (005) RPL_ISUPPORT 
    - [ ] (321) RPL_LISTSTART
    - [ ] (322) RPL_LIST
    - [ ] (323) RPL_LISTEND
    - [ ] (331) RPL_NOTOPIC
    - [ ] (332) RPL_TOPIC
    - [ ] (353) RPL_NAMREPLY
    - [ ] (366) RPL_ENDOFNAMES
    - [ ] (372) RPL_MOTD
    - [ ] (375) RPL_MOTDSTART
    - [ ] (376) RPL_ENDOFMOTD
    - [ ] (392) RPL_USERSSTART
    - [ ] (393) RPL_USERS
    - [ ] (394) RPL_ENDOFUSERS
    - [ ] (401) ERR_NOSUCHNICK
    - [ ] (402) ERR_NOSUCHSERVER
    - [ ] (403) ERR_NOSUCHCHANNEL
    - [ ] (409) ERR_NOORIGIN
    - [ ] (411) ERR_NORECIPIENT
    - [ ] (412) ERR_NOTEXTTOSEND
    - [ ] (421) ERR_UNKNOWNCOMMAND
    - [ ] (431) ERR_NONICKNAMEGIVEN
    - [ ] (436) ERR_NICKCOLLISION
    - [ ] (442) ERR_NOTONCHANNEL
    - [ ] (461) ERR_NEEDMOREPARAMS
    - [ ] (451) ERR_NOTREGISTERED


### Advanced feature
- [ ] maby add async support by using tokio, or mio, not decided yet, or multi-thread

### Other cool to have feature
- [ ] migrate to ircs (6697) and remove support for irc (6667)