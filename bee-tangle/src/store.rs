use crate::vertex::*;

// TODO: move to transaction crate
pub struct TransactionStore {
    // requires ref cell so transactions can be added without requiring a mutable instance, which would
    // not allow other instances such as vertex store to carry a reference
    txs_by_hash : std::cell::RefCell<std::collections::HashMap<String, SharedTransaction>>
}

impl TransactionStore {

    pub fn new() -> Self {
        TransactionStore {
            txs_by_hash : std::cell::RefCell::new(std::collections::HashMap::new())
        }
    }

    pub fn add(&self, tx : SharedTransaction) {
        self.txs_by_hash.borrow_mut().insert(tx.address.to_string(), tx);
    }

    pub fn get_transaction(&self, hash : &String) -> Option<SharedTransaction> {
        match self.txs_by_hash.borrow().get(hash) {
            Some(shared) => Some(shared.clone()),
            None => None
        }
    }
}

pub struct VertexStore<'s> {
    vertices_by_hash : std::collections::HashMap<String, SharedVertex>,
    transaction_store : &'s TransactionStore
}

impl<'s> VertexStore<'s> {

    pub fn new(transaction_store : &'s TransactionStore) -> Self {
        VertexStore {
            vertices_by_hash : std::collections::HashMap::new(),
            transaction_store
        }
    }

    pub fn get_vertex(&self, hash : &String) -> Option<SharedVertex> {
        match self.vertices_by_hash.get(hash) {
            Some(v) => Some(v.clone()),
            None => {
                match self.transaction_store.get_transaction(hash) {
                    Some(tx) => {let v : SharedVertex = tx.into(); Some(v) },
                    None => None
                }
            }
        }
    }
}

mod test {

    use super::*;
    use bee_transaction::*;

    #[test]
    fn test_new_vertex_from_store() {
        let tx_store = TransactionStore::new();
        let vx_store = VertexStore::new(&tx_store);

        let tx = TransactionBuilder::new().address("ABC").build();
        let address = tx.address.to_string().clone();

        match vx_store.get_vertex(&address) {
            Option::None => {}
            _ => { panic!("vertex exists despite transaction not in store"); }
        }

        tx_store.add(tx.into());

        match vx_store.get_vertex(&address) {
            Option::None => { panic!("vertex does not exist despite transaction in store"); }
            Option::Some(vx) => { assert_eq!(address, vx.get_transaction().address, "unexpected vertex") }
        }
    }
}