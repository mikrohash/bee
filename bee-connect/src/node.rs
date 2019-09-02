use std::string::ToString;
use crate::server_socket::ServerSocket;
use crate::client_socket::ClientSocket;
use crate::node;
use crate::message;
use std::time::Duration;

const NOT_CONNECTED : &str = "Node is not connected to this peer.";
const ALREADY_CONNECTED : &str  = "Node is already connected to this peer.";
const NO_TCP : &str  = "NodeID must be set to 'tcp' protocol.";

/// Each [Node](crate::node::Node) instance exposes a TCP socket to allow other nodes to connect to it.
/// Further, each [Node](crate::node::Node) instance can connect to the TCP sockets of other [Node](crate::node::Node) instances.
/// This crate implementation only supports TCP.
/// The interface of this struct is general enough to allow for other protocol implementations.
/// With the help of EEE, multiple protocols can be installed and used by a single bee in parallel.
///
/// # Examples
/// ```
/// use bee_connect::node;
/// // the node_id allows other nodes to connect to your's.
/// let node_id = node::NodeID::new("tcp", "localhost:1337");
/// let node = node::Node::new(&node_id);
/// ```
pub struct Node {
    server_socket : ServerSocket,
    client_sockets : Vec<ClientSocket>,
    timeout : Option<Duration>
}

impl Node {
    /// Creates a new node with a socket listening on the protocol and address specified by the [NodeID](crate::node::NodeID).
    /// Only TCP is supported as protocol in this crate implementation.
    pub fn new(id : &node::NodeID) -> Self {
        id.assert_protocol("tcp");
        let server_socket = ServerSocket::new(id);
        let client_sockets = Vec::new();
        let timeout = Some(Duration::new(3, 0));
        Node { server_socket, client_sockets, timeout }
    }

    /// Connects this node to the node addresses by the `peer_id`.
    /// Both nodes need to connect to each-other to peer successfully.
    /// This crate only supports TCP connections.
    ///
    /// # Errors
    /// * if already connected to that peer
    /// * if peer is not addressed by TCP
    ///
    /// # Examples
    /// ```
    /// use bee_connect::node;
    /// let node1_id = node::NodeID::new("tcp", "localhost:1337");
    /// let node2_id = node::NodeID::new("tcp", "localhost:1338");
    /// let mut node1 = node::Node::new(&node2_id);
    /// let mut node2 = node::Node::new(&node1_id);
    /// node1.connect_to_peer(&node2_id).unwrap();
    /// node2.connect_to_peer(&node1_id).unwrap();
    /// ```
    pub fn connect_to_peer(&mut self, peer_id : &node::NodeID) -> Result<(), String> {
        if &peer_id.protocol != "tcp" {
            Err(NO_TCP.to_string())
        } else if !self.is_connected_to_peer(peer_id) {
            self.add_peer(peer_id);
            Ok(())
        } else {
            Err(ALREADY_CONNECTED.to_string())
        }
    }

    fn add_peer(&mut self, peer_id : &node::NodeID) {
        let client_socket = ClientSocket::new(&peer_id).unwrap();
        client_socket.set_timeout(self.timeout);
        self.client_sockets.push(client_socket);
    }

    /// Disconnects from a peer. Returns `Err` if not connected to that peer.
    ///
    /// # Errors
    /// * if not connected to that peer
    ///
    /// # Examples
    /// ```
    /// # use bee_connect::node;
    /// let node_id = node::NodeID::new("tcp", "localhost:1337");
    /// let mut node = node::Node::new(&node_id);
    ///
    /// let peer_id = node::NodeID::new("tcp", "localhost:1338");
    /// # let mut peer = node::Node::new(&peer_id);
    /// node.connect_to_peer(&peer_id).unwrap();
    /// node.disconnect_from_peer(&peer_id).unwrap();
    /// ```
    pub fn disconnect_from_peer(&mut self, peer_id : &node::NodeID) -> Result<(), String> {
        if self.is_connected_to_peer(peer_id) {
            self.client_sockets.retain(|c| c.get_server_id() != peer_id);
            Ok(())
        } else {
            Err(NOT_CONNECTED.to_string())
        }
    }

    /// Returns a vector containing the [NodeID](crate::node::NodeID) instance of each connected peer.
    pub fn get_peers(&self) -> Vec<node::NodeID> {
        let mut vec = Vec::new();
        for node_id in self.client_sockets.iter().map(|c| c.get_server_id()) {
            vec.push(node_id.clone())
        }
        vec
    }

    /// Returns whether the node is connected to a specific peer.
    ///
    /// # Examples
    /// ```
    /// # use bee_connect::node;
    /// let node_id = node::NodeID::new("tcp", "localhost:1337");
    /// let mut node = node::Node::new(&node_id);
    ///
    /// let peer_id = node::NodeID::new("tcp", "localhost:1338");
    /// # let mut peer = node::Node::new(&peer_id);
    /// node.connect_to_peer(&peer_id).unwrap();
    /// assert!(node.is_connected_to_peer(&peer_id));
    /// ```
    pub fn is_connected_to_peer(&self, peer_id : &node::NodeID) -> bool {
        self.get_peers().contains(peer_id)
    }

    /// Returns the [NodeID](crate::node::NodeID) of this node instance. That NodeID specifies the protocol used and
    /// the address of the exposed [ServerSocket](crate::server_socket::ServerSocket).
    /// # Examples
    /// ```
    /// # use bee_connect::node;
    /// let node_id = node::NodeID::new("tcp", "localhost:1337");
    /// let node = node::Node::new(&node_id);
    /// assert_eq!(node.get_id(), &node_id);
    /// ```
    pub fn get_id(&self) -> &node::NodeID {
        self.server_socket.get_id()
    }

    /// Sets the tolerated timeout for sending messages to peers.
    pub fn set_timeout(&mut self, timeout : Option<Duration>) {
        for client_socket in &self.client_sockets {
            client_socket.set_timeout(timeout.clone());
        }
    }

    /// Sends a message to a connected peer.
    ///
    /// # Errors
    /// * if not connected to that peer
    /// * if connection problems (e.g. timeout)
    pub fn send_message_to_peer(&mut self, peer_id : &node::NodeID, message : &message::Message) -> Result<message::Message, String> {
        let peer = self.client_sockets.iter_mut().find(|c| c.get_server_id() == peer_id);
        match peer {
            Some(client_socket) => {
                client_socket.send_message(message)
            },
            None => Err(NOT_CONNECTED.to_string())
        }
    }

    /// Sends a message to all connected peers.
    pub fn broadcast_message_to_all_peers(&mut self, message : &message::Message) {
        for client_socket in self.client_sockets.iter_mut() {
            client_socket.send_message(message).unwrap();
        }
    }
}

#[derive(Debug)]
pub struct NodeID {
    pub protocol : String,
    pub address : String
}

impl std::cmp::PartialEq for NodeID {
    fn eq(&self, other : &NodeID) -> bool {
        self.protocol == other.protocol && self.address == other.address
    }
}
impl std::clone::Clone for NodeID {
    fn clone(&self) -> Self {
        NodeID {
            protocol : self.protocol.clone(),
            address : self.address.clone()
        }
    }
}

impl NodeID {

    pub fn new(protocol : &str, address : &str) -> Self {
        NodeID {
            protocol : protocol.to_string(),
            address : address.to_string()
        }
    }

    pub fn assert_protocol(&self, protocol : &str) {
        assert_eq!(self.protocol, protocol);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::time::Duration;

    #[test]
    fn test_node_repeated_peering() {
        let mut node = create_tcp_localhost_node(1337);
        let mut peer = create_tcp_localhost_node(1338);
        let peer_id = peer.get_id();

        assert_eq!(node.get_peers(), vec![]);
        node.connect_to_peer(peer_id).unwrap();

        for i in 0..10 {
            assert_eq!(node.get_peers(), vec![peer_id.clone()]);
            node.connect_to_peer(peer_id).unwrap_err();
        }
    }

    #[test]
    fn test_node_un_peer() {
        let mut node = create_tcp_localhost_node(1337);
        let mut peer = create_tcp_localhost_node(1338);
        let peer_id = peer.get_id();

        assert_eq!(node.get_peers(), vec![]);

        for _ in 0..10 {
            node.disconnect_from_peer(peer_id).unwrap_err();
            assert_eq!(node.get_peers(), vec![]);
        }

        node.connect_to_peer(peer_id).unwrap();
        assert_eq!(node.get_peers(), vec![peer_id.clone()]);

        node.disconnect_from_peer(peer_id).unwrap();
        assert_eq!(node.get_peers(), vec![]);

        for _ in 0..10 {
            node.disconnect_from_peer(peer_id).unwrap_err();
            assert_eq!(node.get_peers(), vec![]);
        }
    }

    #[test]
    fn test_node_send_to_peer() {
        let mut node = create_tcp_localhost_node(1337);
        let mut peer = create_tcp_localhost_node(1338);
        let peer_id = &peer.get_id().clone();
        node.connect_to_peer(peer_id).unwrap();

        let peer_handle = std::thread::spawn(move || {
            &peer.server_socket.process_next_request();
        });

        let msg : message::Message = node.send_message_to_peer(peer_id, &message::DISCONNECT_REQUEST).unwrap();
        peer_handle.join().unwrap();
    }

    #[test]
    fn test_node_send_to_peer_timeout() {
        let mut node = create_tcp_localhost_node(1337);
        let mut peer = create_tcp_localhost_node(1338);
        let peer_id = &peer.get_id().clone();
        node.connect_to_peer(peer_id).unwrap();
        node.set_timeout(Some(Duration::new(0, 100000000)));
        node.send_message_to_peer(peer_id, &message::DISCONNECT_REQUEST).unwrap_err();
    }

    #[test]
    fn test_node_peer_with_wrong_protocol() {
        let mut node = create_tcp_localhost_node(1337);
        let peer_id = NodeID::new("https", "iota.org");
        let err = node.connect_to_peer(&peer_id).unwrap_err();
        assert_eq!(err, NO_TCP);
    }

    fn create_tcp_localhost_node(port : u16) -> Node {
        let node_id = node::NodeID { protocol : "tcp".to_string(), address : format!("localhost:{}", port) };
        Node::new(&node_id)
    }

    #[test]
    fn test_close_socket() {
        for _ in 0..10 {
            create_tcp_localhost_node(1337);
        }
    }
}