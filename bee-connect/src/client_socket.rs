use std::string::ToString;
use async_std::net::TcpStream;
use crate::message;
use crate::socket_utils;
use crate::node;
use async_std::task;
use core::borrow::BorrowMut;
use async_std::io::Write;

pub struct ClientSocket {
    stream : TcpStream,
    server_id : node::NodeID
}

impl ClientSocket {
    pub fn new(server_id : &node::NodeID) -> Result<Self, String> {
        server_id.assert_protocol("tcp");

        let stream_result = task::block_on(async {
            TcpStream::connect(&server_id.address).await
        });

        match stream_result {
            Ok(stream) => {
                Ok(ClientSocket{stream, server_id : server_id.clone()})
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn send_message(&mut self, message : &message::Message) -> Result<message::Message, String> {
        task::block_on(async {
            let stream = self.stream.borrow_mut();
            stream.write_all(message).await.unwrap();
            stream.flush().await.unwrap();
            socket_utils::read_message(&mut self.stream)
        })
    }

    pub fn get_server_id(&self) -> &node::NodeID {
        &self.server_id
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_server_id() {
        let server_id = node::NodeID::new("tcp", "localhost:1337");
        let server_socket = crate::server_socket::ServerSocket::new(&server_id);
        let client_socket = ClientSocket::new(&server_id).unwrap();
        let actual_server_id = client_socket.get_server_id();
        assert_eq!(actual_server_id, &server_id);
    }
}