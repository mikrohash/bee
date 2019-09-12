use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use crate::message;
use crate::message::{MessageMetaData, Message, MessageType};

pub fn read_message(stream : & mut TcpStream) -> Result<message::Message, String> {
    match read_into_buffer(stream) {

        Ok(buffer) =>  Ok(Message::new(MessageType::Payload, buffer)),
        Err(e) => Err(e.to_string())
    }
}

fn read_into_buffer(stream : & mut TcpStream) -> Result<Vec<u8>, String> {
    let mut buffer = vec![0; message::MESSAGE_LENGTH];
    match task::block_on( stream.read_exact( &mut buffer)) {
        Ok(()) =>  Ok(buffer),
        Err(e) => Err(e.to_string())
    }
}

fn read_metadata_from_buffer(stream : & mut TcpStream, buffer_size : &mut [u8]) -> Result<MessageMetaData, String> {
    let mut buffer = [0; 2];
    let read_exact_result = task::block_on( stream.read_exact( &mut buffer));
    match read_exact_result {
        Ok(()) =>  MessageMetaData::decode(u16::from_be_bytes(buffer)),
        Err(e) => Err(e.to_string())
    }
}