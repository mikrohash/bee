use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use crate::message;

pub fn read_message(stream : & mut TcpStream) -> Result<message::Message, String> {
    let mut data : message::Message = [0; message::MESSAGE_LENGTH];
    let read_exact_result = task::block_on( stream.read_exact(&mut data));
    match read_exact_result {
        Ok(()) =>  Ok(data),
        Err(e) => Err(e.to_string())
    }
}