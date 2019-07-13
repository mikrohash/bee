#![allow(dead_code)]

use bee_transaction::*;
use bee_tangle::*;

fn main() {
    let tx = TransactionBuilder::new().tag("HELLO").build();
    let vertex_ref : VertexRef = tx.into();

    vertex_ref.print();
    vertex_ref.set_branch(vertex_ref.clone());
    vertex_ref.print();
    vertex_ref.append_child(vertex_ref.clone());
    vertex_ref.append_child(vertex_ref.clone());
    vertex_ref.print();
}