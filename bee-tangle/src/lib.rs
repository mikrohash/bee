pub mod vertex;

pub use crate::vertex::VertexRef;

impl VertexRef {
    pub fn print(&self) {

        fn stringify_opt_vertex_ref(vertex_ref : &Option<VertexRef>) -> String {
            match &vertex_ref {
                Some(vertex_ref) => vertex_ref.get_transaction().address.clone(),
                None => "---".to_string()
            }
        }

        print!("\n=== VERTEX: {} ===\n    branch: {}\n    trunk:  {}",
               self.get_transaction().address.clone(),
               stringify_opt_vertex_ref(&self.get_branch()),
               stringify_opt_vertex_ref(&self.get_trunk()));

        println!("\n--- CHILDREN ({}) ---", self.get_children().len());
        for (i, child) in self.get_children().iter().enumerate() {
            println!("    #{}:     {}", i, child.get_transaction().address.clone());
        }
    }
}