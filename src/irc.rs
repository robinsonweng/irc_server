use std::io::{Read, Write};


pub fn execute<T>(stream: &mut T) -> std::io::Result<()>
where T: Read + Write
{
    let mut buf: [u8; 128] = [0; 128];
    let _ = stream.read(&mut buf);

    let raw_message = String::from_utf8((&buf).to_vec()).unwrap_or_else(|_| {
        panic!(
            "Cant convert message {:?} to utf-8", &buf
        )
    });

    let splited_message = raw_message.split_once(" ");

    if splited_message.is_none() {
        todo!()
    }

    let (command, message) = splited_message.unwrap();

    if command == "CAP" {
        return Ok(());
    }


    println!("incoming command: '{}'", command);
    println!("incoming message: '{}'", message);

    let _ = stream.write(b"cool\r\n");

    Ok(())
}
