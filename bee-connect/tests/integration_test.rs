use std::thread::{JoinHandle};
use bee_connect::client_socket;
use bee_connect::server_socket;
use bee_connect::message;
use bee_connect::node::NodeID;

#[test]
fn test_server_responds_correctly() {
    assert_server_response(message::DISCONNECT_REQUEST, message::DISCONNECT_RESPONSE);
}

fn assert_server_response(request : message::Message, expected_response : message::Message) {
    let server_id = NodeID::new("tcp", "localhost:1337");
    let server_socket_handle = start_server_socket_in_thread(&server_id);
    let mut client_socket = client_socket::ClientSocket::new(&server_id).unwrap();

    let response = client_socket.send_message(&request).unwrap();
    server_socket_handle.join().unwrap();
    assert_eq!(response, expected_response);
}

fn start_server_socket_in_thread(server_id : &NodeID) -> JoinHandle<()> {
    let server_id = server_id.clone();
    std::thread::spawn(move || {
        let mut server_socket = server_socket::ServerSocket::new(&server_id);
        server_socket.listen_to_requests();
    })
}