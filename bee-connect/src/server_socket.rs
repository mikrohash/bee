use std::net::{TcpListener, TcpStream};
use core::borrow::BorrowMut;
use crate::message;
use crate::socket_utils;
use crate::node;
use std::io::Write;

pub struct ServerSocket {
    listener : TcpListener,
    id : node::NodeID
}

impl ServerSocket {

    pub fn new(id : &node::NodeID) -> Self {
        id.assert_protocol("tcp");
        let listener = TcpListener::bind(&id.address).unwrap();
        ServerSocket { listener, id : id.clone() }
    }

    pub fn process_next_request(&mut self) {
        match (&self.listener).incoming().borrow_mut().next() {
            Some(Ok(stream)) => { self.process_ok_stream(stream);  },
            Some(Err(e)) => { panic!("[S] Error: {}", e); },
            None => { /* no open request */ }
        }
    }

    fn process_ok_stream(&self, mut stream : TcpStream) {
        let request = socket_utils::read_message(&mut stream).unwrap();
        let response = self.calculate_response_for_request(&request);
        stream.write_all(&response).unwrap();
        stream.flush().unwrap();
    }

    fn calculate_response_for_request(&self, input : &message::Message) -> message::Message {
        match input {
            &message::DISCONNECT_REQUEST => message::DISCONNECT_RESPONSE,
            _ => message::CONFUSED_RESPONSE
        }
    }

    pub fn get_id(&self) -> &node::NodeID {
        &self.id
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_calculate_response_for_request() {
        assert_server_response(&message::DISCONNECT_REQUEST, &message::DISCONNECT_RESPONSE);
    }

    fn assert_server_response(request : &message::Message, expected_response : &message::Message) {
        let server_id = node::NodeID { protocol : "tcp".to_string(), address : "localhost:1337".to_string() };
        let server_socket = ServerSocket::new(&server_id);
        let response = server_socket.calculate_response_for_request(request);
        assert_eq!(&response, expected_response);
    }

    #[test]
    fn test_close_socket() {
        for _ in 0..10 {
            ServerSocket::new(&node::NodeID::new("tcp", "localhost:1337"));
        }
    }
}