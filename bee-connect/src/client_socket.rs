use std::string::ToString;
use std::net::TcpStream;
use crate::message;
use crate::socket_utils;
use crate::node;
use std::time::Duration;
use std::io::Write;

pub struct ClientSocket {
    stream : TcpStream,
    server_id : node::NodeID
}

impl ClientSocket {
    pub fn new(server_id : &node::NodeID) -> Result<Self, String> {
        server_id.assert_protocol("tcp");
        match TcpStream::connect(&server_id.address) {
            Ok(stream) => {
                Ok(ClientSocket{stream, server_id : server_id.clone()})
            },
            Err(e) => {
                Err(e.to_string())
            }
        }
    }

    pub fn send_message(&mut self, message : &message::Message) -> Result<message::Message, String> {
        self.stream.write_all(message).unwrap();
        self.stream.flush().unwrap();
        socket_utils::read_message(&mut self.stream)
    }

    pub fn get_server_id(&self) -> &node::NodeID {
        &self.server_id
    }

    pub fn set_timeout(&self, timeout : Option<Duration>) {
        self.stream.set_read_timeout(timeout).unwrap();
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