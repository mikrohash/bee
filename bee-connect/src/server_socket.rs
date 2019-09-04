use crate::message;
use crate::socket_utils;
use crate::node;

use async_std::net::{TcpListener, TcpStream};
use async_std::task;
use async_std::prelude::*;

pub struct ServerSocket {
    listener : TcpListener,
    id : node::NodeID
}

impl ServerSocket {

    pub fn new(id : &node::NodeID) -> Self {
        id.assert_protocol("tcp");
        let listener = async_std::task::block_on(
            TcpListener::bind(&id.address)
        ).unwrap();
        ServerSocket { listener, id : id.clone() }
    }

    pub fn listen_to_requests(&mut self) {

        task::block_on(async {
            let mut incoming = self.listener.incoming();
            let polled = incoming.next().await;

            match polled {
                Some(Ok(stream)) => { self.process_stream(stream);  },
                Some(Err(e)) => { panic!("[S] Error: {}", e); },
                None => { /* no open request */ }
            }
        });
    }

    fn process_stream(&self, mut stream : TcpStream) {
        let request = socket_utils::read_message(&mut stream).unwrap();
        let response = self.calculate_response_for_request(&request);

        task::block_on(async {
            stream.write_all(&response).await.unwrap();
            stream.flush().await.unwrap();
        });
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