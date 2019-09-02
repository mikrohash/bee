use std::net::TcpStream;
use std::io::Read;
use crate::message;

pub fn read_message(stream : & mut TcpStream) -> Result<message::Message, String> {
    let mut data : message::Message = [0; message::MESSAGE_LENGTH];
    match stream.read_exact(&mut data) {
        Ok(()) =>  Ok(data),
        Err(e) => Err(e.to_string())
    }
}