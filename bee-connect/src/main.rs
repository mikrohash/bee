use bee_connect::node::{Node, NodeID};

pub fn main() {
    let node_id = NodeID::new("tcp", "0.0.0.0:1337");
    let node = Node::new(&node_id);
}