use bee_transaction::*;

// the actual vertex of which at most one instance exists per transaction in a VertexStore
struct VertexData {
    transaction : Transaction,
    branch : Option<VertexRef>,
    trunk : Option<VertexRef>,
    children : Vec<VertexRef>
}

impl VertexData {
    fn new(transaction : Transaction) -> Self {
        VertexData {
            transaction,
            branch : Option::None,
            trunk : Option::None,
            children : Vec::new()
        }
    }
}

// a multi-access, mutable smart pointer to a vertex
// required so vertices can reference each other bidirectionally
pub struct VertexRef {
    pointer : std::rc::Rc<std::cell::RefCell<VertexData>>
}

impl VertexRef {
    fn borrow_mut(&self) -> std::cell::RefMut<VertexData> {
        self.pointer.borrow_mut()
    }

    pub fn set_branch(&self, branch : VertexRef) {
        assert_eq!(branch.get_transaction().address, self.get_transaction().branch);
        branch.append_child(self.clone());
        self.borrow_mut().branch = Option::Some(branch);
    }

    pub fn set_trunk(&self, trunk : VertexRef) {
        assert_eq!(trunk.get_transaction().address, self.get_transaction().trunk);
        trunk.append_child(self.clone());
        self.borrow_mut().trunk = Option::Some(trunk);
    }

    fn append_child(&self, child : VertexRef) {
        self.borrow_mut().children.push(child);
    }

    pub fn get_children(&self) -> &Vec<VertexRef> {
        &self.borrow().children
    }

    pub fn get_transaction(&self) -> &Transaction {
        &self.borrow().transaction
    }

    pub fn get_branch(&self) -> &Option<VertexRef> {
        &self.borrow().branch
    }

    pub fn get_trunk(&self) -> &Option<VertexRef> {
        &self.borrow().trunk
    }

    // private because VertexData is private; only used internally for convenience
    // therefore not implementing std::borrow::Borrow trait.
    fn borrow(&self) -> &VertexData {
        unsafe {
            self.pointer.as_ptr().as_ref().unwrap()
        }
    }
}

impl std::clone::Clone for VertexRef {
    fn clone(&self) -> VertexRef {
        VertexRef {
            pointer: self.pointer.clone()
        }
    }
}

impl From<Transaction> for VertexRef {
    fn from(transaction : Transaction) -> Self {
        VertexRef{
            pointer : std::rc::Rc::new(std::cell::RefCell::new(VertexData::new(transaction)))
        }
    }
}

pub struct VertexStore {
    vertices_by_hash : std::collections::HashMap<String, VertexRef>
}

impl VertexStore {
    pub fn new() -> Self {
        VertexStore {
            vertices_by_hash : std::collections::HashMap::new()
        }
    }

    pub fn get_vertex(&self, hash : &String) -> Option<&VertexRef> {
        self.vertices_by_hash.get(hash)
    }
}

mod test {

    use super::*;

    #[test]
    fn test_link_correctly() {

        let tx1 = create_tx("", "");
        let tx2 = create_tx("", "");
        let tx3 = create_tx(tx1.address.as_str(), tx2.address.as_str());

        let vertex1 : VertexRef = tx1.into();
        let vertex2 : VertexRef = tx2.into();
        let vertex3 : VertexRef = tx3.into();

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

        let vertex1 : VertexRef = tx1.into();
        let vertex2 : VertexRef = tx2.into();

        vertex2.set_trunk(vertex1.clone());
    }

    fn assert_branch(vertex : &VertexRef, branch : Option<&VertexRef>) {
        if branch.is_none() {
            assert_eq!(true, vertex.get_branch().is_none())
        } else if let Option::Some(inner) = vertex.get_branch() {
            assert_eq!(vertex.get_transaction().branch, branch.unwrap().get_transaction().address, "incorrect branch set for child");
            assert_eq!(inner.get_transaction().address, branch.unwrap().get_transaction().address, "unexpected branch set for child");
        } else {
            panic!("no branch set for child");
        }
    }

    fn assert_trunk(vertex : &VertexRef, trunk : Option<&VertexRef>) {
        if trunk.is_none() {
            assert_eq!(true, vertex.get_trunk().is_none())
        } else if let Option::Some(inner) = vertex.get_trunk() {
            assert_eq!(vertex.get_transaction().trunk, trunk.unwrap().get_transaction().address, "incorrect trunk set for child");
            assert_eq!(inner.get_transaction().address, trunk.unwrap().get_transaction().address, "unexpected trunk set for child");
        } else {
            panic!("no trunk set for child");
        }
    }

    fn assert_children(vertex : &VertexRef, children : &[&VertexRef]) {
        assert_eq!(vertex.get_children().len(), children.len(), "unexpected amount of children");
        for expected_child in children {
            assert_child(vertex, &expected_child);
        }
    }

    fn assert_child(vertex : &VertexRef, child : &VertexRef) {
        for some_child in vertex.get_children() {
            if some_child.get_transaction().address == child.get_transaction().address {
                return;
            }
        }
        panic!("child not found");
    }

    fn create_tx(branch : &str, trunk : &str) -> Transaction {
        let tx_builder = TransactionBuilder::new();
        let random_hash = gen_random_hash();
        println!("HASH: {}", random_hash);
        tx_builder.address(random_hash.as_str())
            .branch(branch)
            .trunk(trunk)
            .build()
    }


    extern crate rand;
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