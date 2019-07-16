#![deny(bad_style, missing_docs, trivial_casts, trivial_numeric_casts, unsafe_code, unstable_features, )]
#![cfg_attr(not(debug_assertions), deny(warnings))]

pub mod vertex;
pub mod store;

pub use crate::vertex::*;
pub use crate::store::*;

impl SharedVertex {
    pub fn print(&self) {

        fn stringify_opt_vertex_ref(vertex_ref : &Option<SharedVertex>) -> String {
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