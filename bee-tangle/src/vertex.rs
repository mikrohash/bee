use bee_transaction::*;

pub struct SharedTransaction {
    pointer : std::rc::Rc<Transaction>
}

impl From<Transaction> for SharedTransaction {
    fn from(transaction : Transaction) -> Self {
        Self {
            pointer : std::rc::Rc::new(transaction)
        }
    }
}

impl std::clone::Clone for SharedTransaction {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer.clone()
        }
    }
}

impl std::ops::Deref for SharedTransaction {
    type Target = Transaction;
    fn deref(&self) -> &Transaction {
        self.pointer.deref()
    }
}

// the actual vertex of which at most one instance exists per transaction in a VertexStore
struct Vertex {
    transaction : SharedTransaction,
    branch : Option<SharedVertex>,
    trunk : Option<SharedVertex>,
    children : Vec<SharedVertex>
}

impl Vertex {
    fn new(transaction : SharedTransaction) -> Self {
        Vertex {
            transaction,
            branch : Option::None,
            trunk : Option::None,
            children : Vec::new()
        }
    }
}

// a multi-access, mutable smart pointer to a vertex
// required so vertices can reference each other bidirectionally
pub struct SharedVertex {
    pointer : std::rc::Rc<std::cell::RefCell<Vertex>>
}

impl SharedVertex {
    fn borrow_mut(&self) -> std::cell::RefMut<Vertex> {
        self.pointer.borrow_mut()
    }

    pub fn set_branch(&self, branch : SharedVertex) {
        assert_eq!(branch.get_transaction().address, self.get_transaction().branch);
        branch.append_child(self.clone());
        self.borrow_mut().branch = Option::Some(branch);
    }

    pub fn set_trunk(&self, trunk : SharedVertex) {
        assert_eq!(trunk.get_transaction().address, self.get_transaction().trunk);
        trunk.append_child(self.clone());
        self.borrow_mut().trunk = Option::Some(trunk);
    }

    fn append_child(&self, child : SharedVertex) {
        self.borrow_mut().children.push(child);
    }

    pub fn get_children(&self) -> &Vec<SharedVertex> {
        &self.borrow().children
    }

    pub fn get_transaction(&self) -> &SharedTransaction {
        &self.borrow().transaction
    }

    pub fn get_branch(&self) -> &Option<SharedVertex> {
        &self.borrow().branch
    }

    pub fn get_trunk(&self) -> &Option<SharedVertex> {
        &self.borrow().trunk
    }

    // private because VertexData is private; only used internally for convenience
    // therefore not implementing std::borrow::Borrow trait.
    fn borrow(&self) -> &Vertex {
        unsafe {
            self.pointer.as_ptr().as_ref().unwrap()
        }
    }
}

impl std::clone::Clone for SharedVertex {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer.clone()
        }
    }
}

impl From<SharedTransaction> for SharedVertex {
    fn from(transaction : SharedTransaction) -> Self {
        Self {
            pointer : std::rc::Rc::new(std::cell::RefCell::new(Vertex::new(transaction)))
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_link_correctly() {

        let tx1 = create_tx("", "");
        let tx2 = create_tx("", "");
        let tx3 = create_tx(tx1.address.as_str(), tx2.address.as_str());

        let vertex1 : SharedVertex = tx1.into();
        let vertex2 : SharedVertex = tx2.into();
        let vertex3 : SharedVertex = tx3.into();

        vertex3.set_branch(vertex1.clone());
        vertex3.set_trunk(vertex2.clone());

        assert_children(&vertex1, &[&vertex3]);
        assert_branch(&vertex1, Option::None);
        assert_trunk(&vertex1, Option::None);

        assert_children(&vertex2, &[&vertex3]);
        assert_branch(&vertex2, Option::None);
        assert_trunk(&vertex2, Option::None);

        assert_children(&vertex3, &[]);
        assert_branch(&vertex3, Option::Some(&vertex1));
        assert_trunk(&vertex3, Option::Some(&vertex2));
    }

    #[test]
    #[should_panic]
    fn test_link_incorrectly() {

        let tx1 = create_tx("", "");
        let tx2 = create_tx(tx1.address.as_str(), "");

        let vertex1 : SharedVertex = tx1.into();
        let vertex2 : SharedVertex = tx2.into();

        vertex2.set_trunk(vertex1.clone());
    }

    fn assert_branch(vertex : &SharedVertex, branch : Option<&SharedVertex>) {
        if branch.is_none() {
            assert_eq!(true, vertex.get_branch().is_none())
        } else if let Option::Some(inner) = vertex.get_branch() {
            assert_eq!(vertex.get_transaction().branch, branch.unwrap().get_transaction().address, "incorrect branch set for child");
            assert_eq!(inner.get_transaction().address, branch.unwrap().get_transaction().address, "unexpected branch set for child");
        } else {
            panic!("no branch set for child");
        }
    }

    fn assert_trunk(vertex : &SharedVertex, trunk : Option<&SharedVertex>) {
        if trunk.is_none() {
            assert_eq!(true, vertex.get_trunk().is_none())
        } else if let Option::Some(inner) = vertex.get_trunk() {
            assert_eq!(vertex.get_transaction().trunk, trunk.unwrap().get_transaction().address, "incorrect trunk set for child");
            assert_eq!(inner.get_transaction().address, trunk.unwrap().get_transaction().address, "unexpected trunk set for child");
        } else {
            panic!("no trunk set for child");
        }
    }

    fn assert_children(vertex : &SharedVertex, children : &[&SharedVertex]) {
        assert_eq!(vertex.get_children().len(), children.len(), "unexpected amount of children");
        for expected_child in children {
            assert_child(vertex, &expected_child);
        }
    }

    fn assert_child(vertex : &SharedVertex, child : &SharedVertex) {
        for some_child in vertex.get_children() {
            if some_child.get_transaction().address == child.get_transaction().address {
                return;
            }
        }
        panic!("child not found");
    }

    fn create_tx(branch : &str, trunk : &str) -> SharedTransaction {
        let tx_builder = TransactionBuilder::new();
        let random_hash = gen_random_hash();
        println!("HASH: {}", random_hash);
        tx_builder.address(random_hash.as_str())
            .branch(branch)
            .trunk(trunk)
            .build()
            .into()
    }

    use rand::Rng;

    fn gen_random_hash() -> String {

        let mut rng = rand::thread_rng();
        let mut chars: [char; 81] = ['9'; 81];

        for i in 0..81 {
            let rand = rng.gen_range(0, 27);
            chars[i] = match rand {
                0 => '9',
                x => ('A' as u8 +(x as u8)-1) as char
            };
        }

        chars.iter().cloned().collect::<String>()
    }
}