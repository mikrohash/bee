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

    pub fn append_child(&self, child : VertexRef) {
        self.borrow_mut().children.push(child);
    }

    pub fn set_branch(&self, branch : VertexRef) {
        self.borrow_mut().branch = Option::Some(branch);
    }

    pub fn set_trunk(&self, trunk : VertexRef) {
        self.borrow_mut().trunk = Option::Some(trunk);
    }

    pub fn get_children(&self) -> &Vec<VertexRef> {
        &self.deref().children
    }

    pub fn get_transaction(&self) -> &Transaction {
        &self.deref().transaction
    }

    pub fn get_branch(&self) -> &Option<VertexRef> {
        &self.deref().branch
    }

    pub fn get_trunk(&self) -> &Option<VertexRef> {
        &self.deref().trunk
    }

    // private because VertexData is private
    // only used internally for convenience
    // therefore not implementing std::ops::Deref trait.
    fn deref(&self) -> &VertexData {
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