# TODO

## Main feature
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


## Advanced feature
- [ ] maby add async support by using tokio, or mio, not decided yet, or multi-thread

## Other cool to have feature
- [ ] migrate to ircs (6697) and remove support for irc (6667)