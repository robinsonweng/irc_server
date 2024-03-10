use std::io::{Read, Write};


pub fn execute<T>(stream: &mut T) -> std::io::Result<()>
where T: Read + Write
{
    let mut buf: [u8; 128] = [0; 128];
    let raw_message = stream.read(&mut buf);

    let message = String::from_utf8((&buf).to_vec()).unwrap_or_else(|_| {
        panic!(
            "Cant convert message {:?} to utf-8", &raw_message
        )
    });

    let _ = stream.write(b"cool");

    Ok(())
}
