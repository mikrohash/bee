use crate::vertex::*;

/// Provides access to all transactions known by the bee node. New transactions will be added and
/// old transactions removed when pruned.
// TODO: move to transaction crate
pub struct TransactionStore {
    // requires ref cell so transactions can be added without requiring a mutable instance, which would
    // not allow other instances such as vertex store to carry a reference
    txs_by_hash : std::cell::RefCell<std::collections::HashMap<String, SharedTransaction>>
}

impl TransactionStore {

    /// Creates a new, empty transaction store.
    /// # Example
    /// ```
    /// # use bee_tangle::store::TransactionStore;
    /// let tx_store = TransactionStore::new();
    /// ```
    pub fn new() -> Self {
        TransactionStore {
            txs_by_hash : std::cell::RefCell::new(std::collections::HashMap::new())
        }
    }

    /// Adds a transaction to the store.
    /// # Example
    /// ```
    /// # use bee_tangle::*;
    /// # use bee_transaction::TransactionBuilder;
    /// let tx_store = TransactionStore::new();
    /// let tx = TransactionBuilder::new().address("ABC").build();
    /// tx_store.add(tx.into());
    /// ```
    pub fn add(&self, tx : SharedTransaction) {
        self.txs_by_hash.borrow_mut().insert(tx.address.to_string(), tx);
    }

    /// Finds a transaction by its hash from the store.
    /// # Example
    /// ```
    /// # use bee_tangle::*;
    /// # use bee_transaction::TransactionBuilder;
    /// let tx_store = TransactionStore::new();
    /// let tx = TransactionBuilder::new().address("ABC").build();
    /// tx_store.add(tx.into());
    ///
    /// let found = tx_store.get_transaction("ABC").unwrap();
    /// ```
    pub fn get_transaction(&self, hash : &str) -> Option<SharedTransaction> {
        match self.txs_by_hash.borrow().get(&hash.to_string()) {
            Some(shared) => Some(shared.clone()),
            None => None
        }
    }
}

/// Provides access to all vertices belonging to the same DAG. New vertices will be added when
/// new transactions arrive and be linked with other transactions according to their trunk/branch.
/// Vertices will be removed when the underlying transactions are pruned away.
pub struct VertexStore<'s> {
    vertices_by_hash : std::collections::HashMap<String, SharedVertex>,
    transaction_store : &'s TransactionStore
}

impl<'s> VertexStore<'s> {

    /// Creates a new vertex store. The `transaction_store` argument provides the access points for
    /// transactions to be referred to by the vertices.
    /// # Example
    /// ```
    /// # use bee_tangle::store::*;
    /// let tx_store = TransactionStore::new();
    /// let vx_store = VertexStore::new(&tx_store);
    /// ```
    pub fn new(transaction_store : &'s TransactionStore) -> Self {
        VertexStore {
            vertices_by_hash : std::collections::HashMap::new(),
            transaction_store
        }
    }

    /// Get the vertex instance by the hash of the corresponding transaction. Returns an option, which
    /// is None if the vertex does not exist because no transaction with the requested hash does exist.
    /// # Example
    /// ```
    /// # use bee_tangle::*;
    /// # use bee_transaction::TransactionBuilder;
    /// let tx_store = TransactionStore::new();
    /// let vx_store = VertexStore::new(&tx_store);
    ///
    /// let tx = TransactionBuilder::new().address("ABC").build();
    /// tx_store.add(tx.into());
    ///
    /// let vertex = vx_store.get_vertex("ABC").unwrap();
    /// ```
    pub fn get_vertex(&self, hash : &str) -> Option<SharedVertex> {
        match self.vertices_by_hash.get(&hash.to_string()) {
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

#[cfg(test)]
mod test {

    use crate::store::*;
    use bee_transaction::*;

    #[test]
    fn test_new_vertex_from_store() {
        let tx_store = TransactionStore::new();
        let vx_store = VertexStore::new(&tx_store);

        let address = "ABC";
        let tx = TransactionBuilder::new().address(address).build();

        match vx_store.get_vertex(address) {
            Option::None => {}
            _ => { panic!("vertex exists despite transaction not in store"); }
        }

        tx_store.add(tx.into());

        match vx_store.get_vertex(address) {
            Option::None => { panic!("vertex does not exist despite transaction in store"); }
            Option::Some(vx) => { assert_eq!(address, vx.get_transaction().address, "unexpected vertex") }
        }
    }
}